use super::*;
use std::{
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
    time::Duration,
};

fn options() -> RequestOptions {
    RequestOptions::with_timeout(Duration::from_secs(5)).unwrap()
}
fn pages() -> Pages<'static, u32, u64> {
    Pages::new(0, options(), Session::new(), 10, |cursor, _| async move {
        Ok(match cursor {
            0 => Page {
                items: vec![1, 1, 2],
                next: Some(3),
            },
            3 => Page {
                items: vec![3, 4],
                next: None,
            },
            _ => return Err(Error::new(ErrorKind::Protocol)),
        })
    })
    .unwrap()
}
#[tokio::test]
async fn page_and_item_iteration_preserve_order_and_duplicate_rows() {
    let mut stream = pages();
    assert_eq!(
        stream.next_page().await.unwrap().unwrap().items,
        vec![1, 1, 2]
    );
    assert_eq!(
        stream.resume(),
        Some(Resume {
            cursor: 3,
            item_offset: 0
        })
    );
    assert_eq!(stream.next_page().await.unwrap().unwrap().items, vec![3, 4]);
    assert!(stream.next_page().await.unwrap().is_none());
    let mut stream = pages().items();
    let mut values = Vec::new();
    while let Some(value) = stream.next().await.unwrap() {
        values.push(value);
    }
    assert_eq!(values, vec![1, 1, 2, 3, 4]);
    assert!(stream.resume().is_none());
}
#[tokio::test]
async fn item_cap_preserves_mid_page_resume_without_an_extra_fetch() {
    let limits = CollectOptions {
        max_items: 2,
        ..CollectOptions::default()
    };
    let result = pages().collect(limits).await.unwrap();
    assert_eq!(result.items, vec![1, 1]);
    assert_eq!(result.completion, Completion::Truncated(Truncation::Items));
    assert_eq!(result.pages_read, 1);
    assert_eq!(
        result.resume,
        Some(Resume {
            cursor: 0,
            item_offset: 2
        })
    );
    let remainder = pages()
        .items()
        .resume_from(result.resume.unwrap())
        .unwrap()
        .collect(CollectOptions::default())
        .await
        .unwrap();
    assert_eq!(remainder.items, vec![2, 3, 4]);
    assert!(remainder.is_complete());
}
#[tokio::test]
async fn exact_final_item_cap_reports_complete_without_more_network_work() {
    let limits = CollectOptions {
        max_items: 5,
        ..CollectOptions::default()
    };
    let result = pages().collect(limits).await.unwrap();
    assert!(result.is_complete());
    assert!(result.resume.is_none());
    assert_eq!(result.pages_read, 2);
}
#[tokio::test]
async fn byte_cap_counts_serialized_array_framing_and_retains_unconsumed_item() {
    let stream = Pages::new(7, options(), Session::new(), 10, |_, _| async {
        Ok(Page::<_, u64> {
            items: vec![1, 22],
            next: None,
        })
    })
    .unwrap();
    let limits = CollectOptions {
        max_bytes: 5,
        ..CollectOptions::default()
    };
    let result = stream.collect(limits).await.unwrap();
    assert_eq!(result.items, vec![1]);
    assert_eq!(result.wire_bytes, 3);
    assert_eq!(result.completion, Completion::Truncated(Truncation::Bytes));
    assert_eq!(
        result.resume,
        Some(Resume {
            cursor: 7,
            item_offset: 1
        })
    );
}
#[tokio::test]
async fn page_cap_has_explicit_truncation_and_next_page_resume() {
    let limits = CollectOptions {
        max_pages: 1,
        ..CollectOptions::default()
    };
    let result = pages().collect(limits).await.unwrap();
    assert_eq!(result.items, vec![1, 1, 2]);
    assert_eq!(result.pages_read, 1);
    assert_eq!(result.completion, Completion::Truncated(Truncation::Pages));
    assert_eq!(
        result.resume,
        Some(Resume {
            cursor: 3,
            item_offset: 0
        })
    );
}
#[tokio::test]
async fn repeated_cursors_never_trigger_a_duplicate_request_and_keep_partial_rows() {
    let calls = Arc::new(AtomicUsize::new(0));
    let count = Arc::clone(&calls);
    let stream = Pages::new(
        "private-cursor".to_owned(),
        options(),
        Session::new(),
        10,
        move |cursor, _| {
            count.fetch_add(1, Ordering::SeqCst);
            async move {
                Ok(Page {
                    items: vec!["private-row".to_owned()],
                    next: Some(cursor),
                })
            }
        },
    )
    .unwrap();
    let error = stream.collect(CollectOptions::default()).await.unwrap_err();
    assert_eq!(calls.load(Ordering::SeqCst), 1);
    assert_eq!(error.error().kind(), ErrorKind::Pagination);
    assert_eq!(error.partial().items, vec!["private-row"]);
    assert!(error.partial().resume.is_some());
    let display = format!("{error:?} {error}");
    assert!(!display.contains("private-row"));
    assert!(!display.contains("private-cursor"));
}
#[tokio::test]
async fn mid_walk_remote_failure_retains_accepted_data_and_source_error() {
    let stream = Pages::new(0, options(), Session::new(), 10, |cursor, _| async move {
        if cursor == 0 {
            Ok(Page {
                items: vec![1],
                next: Some(1),
            })
        } else {
            Err(Error::new(ErrorKind::Http)
                .with_status(503)
                .with_outcome(crate::RemoteOutcome::Unknown))
        }
    })
    .unwrap();
    let (error, partial) = stream
        .collect(CollectOptions::default())
        .await
        .unwrap_err()
        .into_parts();
    assert_eq!(partial.items, vec![1]);
    assert_eq!(partial.completion, Completion::Interrupted);
    assert_eq!(
        partial.resume,
        Some(Resume {
            cursor: 1,
            item_offset: 0
        })
    );
    assert_eq!(error.status(), Some(503));
    assert_eq!(error.remote_outcome(), crate::RemoteOutcome::Unknown);
}
#[tokio::test]
async fn empty_final_pages_succeed_and_empty_nonfinal_pages_fail() {
    let stream = Pages::new(0, options(), Session::new(), 10, |_, _| async {
        Ok(Page::<u32, u64> {
            items: vec![],
            next: None,
        })
    })
    .unwrap();
    assert!(
        stream
            .collect(CollectOptions::default())
            .await
            .unwrap()
            .is_complete()
    );
    let stream = Pages::new(0, options(), Session::new(), 10, |_, _| async {
        Ok(Page::<u32, u64> {
            items: vec![],
            next: Some(1),
        })
    })
    .unwrap();
    assert_eq!(
        stream
            .collect(CollectOptions::default())
            .await
            .unwrap_err()
            .error()
            .kind(),
        ErrorKind::Pagination
    );
}
#[tokio::test]
async fn buffered_items_honor_per_call_and_session_cancellation() {
    let opts = options();
    let session = Session::new();
    let mut stream = Pages::new(0, opts.clone(), session.clone(), 10, |_, _| async {
        Ok(Page::<_, u64> {
            items: vec![1, 2],
            next: None,
        })
    })
    .unwrap()
    .items();
    assert_eq!(stream.next().await.unwrap(), Some(1));
    opts.cancel();
    assert_eq!(
        stream.next().await.unwrap_err().kind(),
        ErrorKind::Cancelled
    );
    assert_eq!(
        stream.resume(),
        Some(Resume {
            cursor: 0,
            item_offset: 1
        })
    );
    let mut stream = Pages::new(0, options(), session.clone(), 10, |_, _| async {
        Ok(Page::<_, u64> {
            items: vec![1, 2],
            next: None,
        })
    })
    .unwrap()
    .items();
    assert_eq!(stream.next().await.unwrap(), Some(1));
    session.cancel();
    assert_eq!(
        stream.next().await.unwrap_err().kind(),
        ErrorKind::Cancelled
    );
}
#[tokio::test(start_paused = true)]
async fn all_pages_share_one_absolute_deadline() {
    let opts = RequestOptions::with_timeout(Duration::from_secs(3)).unwrap();
    let deadline = opts.deadline();
    let stream = Pages::new(
        0,
        opts,
        Session::new(),
        10,
        move |cursor, opts| async move {
            assert_eq!(opts.deadline(), deadline);
            tokio::time::sleep(Duration::from_secs(2)).await;
            Ok(Page {
                items: vec![cursor],
                next: Some(cursor + 1),
            })
        },
    )
    .unwrap();
    let error = stream.collect(CollectOptions::default()).await.unwrap_err();
    assert_eq!(error.error().kind(), ErrorKind::Deadline);
    assert_eq!(error.partial().items, vec![0]);
    assert_eq!(tokio::time::Instant::now(), deadline);
}
#[tokio::test]
async fn invalid_resume_and_invalid_collection_limits_are_rejected() {
    let stream = pages()
        .items()
        .resume_from(Resume {
            cursor: 0,
            item_offset: 4,
        })
        .unwrap();
    assert_eq!(
        stream
            .collect(CollectOptions::default())
            .await
            .unwrap_err()
            .error()
            .kind(),
        ErrorKind::Pagination
    );
    let opts = CollectOptions {
        max_items: 0,
        ..CollectOptions::default()
    };
    assert_eq!(
        pages().collect(opts).await.unwrap_err().error().kind(),
        ErrorKind::Validation
    );
    assert!(
        Pages::<u32, u64>::new(0, options(), Session::new(), 0, |_, _| async {
            Ok(Page {
                items: vec![],
                next: None,
            })
        })
        .is_err()
    );
}

#[tokio::test]
async fn resuming_after_the_last_item_in_a_page_obeys_the_collection_page_cap() {
    let stream = pages()
        .items()
        .resume_from(Resume {
            cursor: 0,
            item_offset: 3,
        })
        .unwrap();
    let limits = CollectOptions {
        max_pages: 1,
        ..CollectOptions::default()
    };
    let result = stream.collect(limits).await.unwrap();
    assert!(result.items.is_empty());
    assert_eq!(result.pages_read, 1);
    assert_eq!(result.completion, Completion::Truncated(Truncation::Pages));
    assert_eq!(
        result.resume,
        Some(Resume {
            cursor: 3,
            item_offset: 0
        })
    );
}

#[tokio::test]
async fn failed_iterators_cannot_later_claim_a_complete_collection() {
    let mut stream = Pages::new(0, options(), Session::new(), 10, |_, _| async {
        Err::<Page<u32, u64>, _>(Error::new(ErrorKind::Transport).with_status(502))
    })
    .unwrap()
    .items();
    assert_eq!(
        stream.next().await.unwrap_err().kind(),
        ErrorKind::Transport
    );
    let error = stream.collect(CollectOptions::default()).await.unwrap_err();
    assert_eq!(error.error().status(), Some(502));
    assert_eq!(error.partial().completion, Completion::Interrupted);
}

#[tokio::test]
async fn local_cancellation_is_latched_even_after_consuming_the_last_buffered_item() {
    let opts = options();
    let mut stream = Pages::new(0, opts.clone(), Session::new(), 10, |_, _| async {
        Ok(Page::<_, u64> {
            items: vec![1],
            next: None,
        })
    })
    .unwrap()
    .items();
    assert_eq!(stream.next().await.unwrap(), Some(1));
    opts.cancel();
    assert_eq!(
        stream.next().await.unwrap_err().kind(),
        ErrorKind::Cancelled
    );
    assert_eq!(
        stream
            .collect(CollectOptions::default())
            .await
            .unwrap_err()
            .error()
            .kind(),
        ErrorKind::Cancelled
    );
}

#[tokio::test(start_paused = true)]
async fn inactive_walks_never_invoke_the_fetch_callback() {
    for expired in [false, true] {
        let opts = RequestOptions::with_timeout(Duration::from_secs(1)).unwrap();
        if expired {
            tokio::time::advance(Duration::from_secs(1)).await;
        } else {
            opts.cancel();
        }
        let calls = Arc::new(AtomicUsize::new(0));
        let counter = calls.clone();
        let mut stream = Pages::new(0, opts, Session::new(), 10, move |_, _| {
            counter.fetch_add(1, Ordering::SeqCst);
            async {
                Ok(Page::<u32, u64> {
                    items: vec![],
                    next: None,
                })
            }
        })
        .unwrap();
        assert_eq!(
            stream.next_page().await.unwrap_err().kind(),
            if expired {
                ErrorKind::Deadline
            } else {
                ErrorKind::Cancelled
            }
        );
        assert_eq!(calls.load(Ordering::SeqCst), 0);
        assert!(stream.seen.is_empty());
    }
}

#[tokio::test]
async fn cursor_values_and_retained_history_have_independent_bounds() {
    let oversized = "x".repeat(4096);
    let invalid = Pages::new(
        oversized.clone(),
        options(),
        Session::new(),
        10,
        |_, _| async {
            Ok(Page::<u32, String> {
                items: vec![],
                next: None,
            })
        },
    );
    assert_eq!(invalid.unwrap_err().kind(), ErrorKind::Limit);
    let stream = Pages::new(
        "small".to_owned(),
        options(),
        Session::new(),
        10,
        move |_, _| {
            let next = oversized.clone();
            async move {
                Ok(Page {
                    items: vec![1],
                    next: Some(next),
                })
            }
        },
    )
    .unwrap();
    let error = stream.collect(CollectOptions::default()).await.unwrap_err();
    assert_eq!(error.error().kind(), ErrorKind::Limit);
    assert_eq!(error.partial().items, vec![1]);
    assert_eq!(error.partial().pages_read, 1);
    assert_eq!(error.partial().resume.as_ref().unwrap().cursor, "small");
    assert_eq!(error.partial().resume.as_ref().unwrap().item_offset, 1);
    let limits = CursorLimits {
        max_bytes: 4,
        max_retained_bytes: 4,
    };
    let stream = Pages::new_with_cursor_limits(
        0,
        options(),
        Session::new(),
        10,
        limits,
        |cursor, _| async move {
            Ok(Page {
                items: vec![cursor],
                next: Some(cursor + 1),
            })
        },
    )
    .unwrap();
    let error = stream.collect(CollectOptions::default()).await.unwrap_err();
    assert_eq!(error.error().kind(), ErrorKind::Limit);
    assert_eq!(error.partial().items, vec![0, 1, 2, 3]);
    assert_eq!(
        error.partial().resume,
        Some(Resume {
            cursor: 4,
            item_offset: 0
        })
    );
}

#[tokio::test]
async fn rejected_next_cursor_retains_rows_and_can_resume_with_larger_bounds() {
    let long = "opaque".repeat(1024);
    let make = || {
        let long = long.clone();
        Pages::new(
            "start".to_owned(),
            options(),
            Session::new(),
            10,
            move |cursor, _| {
                let long = long.clone();
                async move {
                    Ok(if cursor == "start" {
                        Page {
                            items: vec![1_u32, 2],
                            next: Some(long),
                        }
                    } else {
                        Page {
                            items: vec![3],
                            next: None,
                        }
                    })
                }
            },
        )
        .unwrap()
    };
    let failure = make().collect(CollectOptions::default()).await.unwrap_err();
    assert_eq!(failure.error().kind(), ErrorKind::Limit);
    assert_eq!(failure.partial().items, vec![1, 2]);
    assert_eq!(failure.partial().pages_read, 1);
    let resume = failure.partial().resume.clone().unwrap();
    assert_eq!(
        resume,
        Resume {
            cursor: "start".into(),
            item_offset: 2
        }
    );
    let limits = CursorLimits {
        max_bytes: 16 * 1024,
        ..CursorLimits::default()
    };
    let result = make()
        .with_cursor_limits(limits)
        .unwrap()
        .items()
        .resume_from(resume)
        .unwrap()
        .collect(CollectOptions::default())
        .await
        .unwrap();
    assert_eq!(result.items, vec![3]);
    assert!(result.is_complete());

    let mut items = make().items();
    assert_eq!(items.next().await.unwrap(), Some(1));
    assert_eq!(items.next().await.unwrap(), Some(2));
    assert_eq!(items.next().await.unwrap_err().kind(), ErrorKind::Limit);
    assert_eq!(items.resume().unwrap().item_offset, 2);
    assert_eq!(items.next().await.unwrap_err().kind(), ErrorKind::Limit);

    let mut pages = make();
    let page = pages.next_page().await.unwrap().unwrap();
    assert_eq!(page.items, vec![1, 2]);
    assert_eq!(page.next, Some(long));
    assert_eq!(pages.resume().unwrap().item_offset, 2);
    assert_eq!(
        pages.next_page().await.unwrap_err().kind(),
        ErrorKind::Limit
    );
    assert_eq!(
        pages.with_cursor_limits(limits).unwrap_err().kind(),
        ErrorKind::Validation
    );
}

#[tokio::test]
async fn rejected_cursor_errors_remain_visible_at_collection_limits() {
    for byte_limit in [false, true] {
        let stream = Pages::new(
            "start".to_owned(),
            options(),
            Session::new(),
            10,
            |_, _| async {
                Ok(Page {
                    items: vec![1_u32, 2],
                    next: Some("x".repeat(4096)),
                })
            },
        )
        .unwrap();
        let collect = if byte_limit {
            CollectOptions::default().with_max_bytes(3).unwrap()
        } else {
            CollectOptions::default().with_max_items(1).unwrap()
        };
        let failure = stream.collect(collect).await.unwrap_err();
        assert_eq!(failure.error().kind(), ErrorKind::Limit);
        assert_eq!(failure.partial().completion, Completion::Interrupted);
        assert_eq!(failure.partial().items, vec![1]);
        assert_eq!(failure.partial().resume.as_ref().unwrap().item_offset, 1);
        assert_eq!(failure.partial().pages_read, 1);
    }
}
