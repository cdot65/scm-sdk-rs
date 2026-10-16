//! Prisma AIRS product namespaces. Client availability is tracked independently
//! from the shared typed contracts available through the `models` feature.
pub mod management;
pub mod scan;
pub mod model_security {
    pub mod models {
        pub use crate::models::model_security::*;
    }
}
pub mod red_team {
    pub mod models {
        pub use crate::models::red_team::*;
    }
}
pub mod gateway;
pub mod agentguard {
    pub mod models {
        pub use crate::models::agentguard::*;
    }
}
