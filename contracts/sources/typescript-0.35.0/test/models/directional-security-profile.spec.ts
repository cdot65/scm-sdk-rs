import { afterEach, describe, expect, expectTypeOf, it, vi } from 'vitest';
import {
  CreateSecurityProfileRequestSchema,
  ContentTypeConfigurationSchema,
  ContentTypeConfigurationsSchema,
  AiSecurityProfileSchema,
  ModelConfigurationSchema,
  ManagementClient,
  SecurityProfileSchema,
  ErrorType,
  type ContentTypeConfiguration,
  type ContentTypeConfigurations,
} from '../../src/index.js';
import fixture from './fixtures/directional-security-profile.json';

// Sanitized complete POST response supplied on 2026-10-09. The supplied GET
// response has the same policy and omits dlp_tenant_id; the POST request was truncated.
const directions = ['prompt', 'response', 'tool-call', 'tool-response'] as const;

function client() {
  return new ManagementClient({
    clientId: 'fixture-client',
    clientSecret: 'fixture-secret',
    tsgId: '100',
    apiEndpoint: 'https://fixture.invalid/aisec',
    tokenEndpoint: 'https://fixture.invalid/oauth2/token',
    numRetries: 0,
  });
}

describe('observed directional security profiles', () => {
  afterEach(() => vi.unstubAllGlobals());

  it('preserves the complete POST and GET response shapes without inserting defaults', () => {
    expect(SecurityProfileSchema.parse(fixture)).toEqual(fixture);
    const getResponse = structuredClone(fixture);
    Reflect.deleteProperty(getResponse, 'dlp_tenant_id');
    expect(SecurityProfileSchema.parse(getResponse)).toEqual(getResponse);
    expect(CreateSecurityProfileRequestSchema.parse(fixture)).toEqual(fixture);
  });

  it.each(directions)(
    'validates detector severities inside %s before authentication',
    async (direction) => {
      const body = structuredClone(fixture);
      Reflect.set(
        body.policy['ai-security-profiles'][0]['content-type-configurations'][direction][
          'model-protection'
        ][0],
        'severity',
        123,
      );
      const fetch = vi.fn();
      vi.stubGlobal('fetch', fetch);
      await expect(client().profiles.create(body)).rejects.toMatchObject({
        errorType: ErrorType.USER_REQUEST_PAYLOAD_ERROR,
      });
      expect(fetch).not.toHaveBeenCalled();
    },
  );

  it('exposes explicit types for direction settings and optional metadata', () => {
    const parsed = SecurityProfileSchema.parse(fixture);
    const ai = parsed.policy?.['ai-security-profiles']?.[0];
    expectTypeOf(ai?.['content-type-configurations']).toEqualTypeOf<
      ContentTypeConfigurations | undefined
    >();
    const prompt = ai?.['content-type-configurations']?.prompt;
    expectTypeOf(prompt).toEqualTypeOf<ContentTypeConfiguration | undefined>();
    expectTypeOf(prompt?.['model-protection']?.[0]?.severity).toEqualTypeOf<string | undefined>();
    expectTypeOf(ai?.['content-type-mode']).toEqualTypeOf<string | undefined>();
    expectTypeOf(
      ai?.['model-configuration']?.['enable-full-conversation-inspection'],
    ).toEqualTypeOf<boolean | undefined>();
    expectTypeOf(parsed.dlp_tenant_id).toEqualTypeOf<string | undefined>();
    expectTypeOf(CreateSecurityProfileRequestSchema.parse(fixture).dlp_tenant_id).toEqualTypeOf<
      string | undefined
    >();
  });

  it('preserves omissions, legacy settings, and future additive fields', () => {
    expect(ContentTypeConfigurationSchema.parse({})).toEqual({});
    expect(ContentTypeConfigurationsSchema.parse({})).toEqual({});
    expect(ModelConfigurationSchema.parse({})).toEqual({});
    const legacy = {
      'model-configuration': { 'model-protection': [{ name: 'prompt-injection' }] },
    };
    expect(AiSecurityProfileSchema.parse(legacy)).toEqual(legacy);
    const future = {
      'content-type-mode': 'future-mode',
      'content-type-configurations': {
        prompt: { 'future-detector': { action: 'observe' } },
        'future-direction': { enabled: true },
      },
    };
    expect(AiSecurityProfileSchema.parse(future)).toEqual(future);
  });

  it.each([
    { 'content-type-mode': 123 },
    { 'content-type-configurations': null },
    { 'content-type-configurations': [] },
    { 'content-type-configurations': { prompt: null } },
    { 'content-type-configurations': { response: [] } },
    {
      'content-type-configurations': {
        'tool-call': {
          'data-protection': {
            'database-security': [{ name: 'db', action: 'block', severity: false }],
          },
        },
      },
    },
    {
      'content-type-configurations': {
        'tool-response': { 'model-protection': [{ 'severity-by-confidence': { high: 123 } }] },
      },
    },
    {
      'content-type-configurations': {
        prompt: { 'app-protection': { 'url-detected-severity': false } },
      },
    },
    {
      'content-type-configurations': {
        response: { 'agent-protection': [{ name: 'agent', action: 'block', severity: 123 }] },
      },
    },
    {
      'content-type-configurations': {
        'tool-call': {
          'data-protection': {
            'data-leak-detection': { action: 'block', 'mask-data-inline': 'true' },
          },
        },
      },
    },
    { 'model-configuration': { 'enable-full-conversation-inspection': 'false' } },
  ])('rejects malformed explicitly modeled extensions: %j', (ai) => {
    const body = { profile_name: 'test', policy: { 'ai-security-profiles': [ai] } };
    expect(CreateSecurityProfileRequestSchema.safeParse(body).success).toBe(false);
    expect(SecurityProfileSchema.safeParse(body).success).toBe(false);
  });

  it('rejects malformed DLP tenant identifiers without coercion', () => {
    const profile = { ...fixture, dlp_tenant_id: 123 };
    expect(CreateSecurityProfileRequestSchema.safeParse(profile).success).toBe(false);
    expect(SecurityProfileSchema.safeParse(profile).success).toBe(false);
  });

  it('retains every direction through OAuth create, get, and read-modify-write update', async () => {
    const getResponse = structuredClone(fixture);
    Reflect.deleteProperty(getResponse, 'dlp_tenant_id');
    const writes: Array<{ method: string; body: unknown }> = [];
    const fetch = vi.fn(async (url: string, init: RequestInit) => {
      if (String(url).includes('/oauth2/token')) {
        return new Response(JSON.stringify({ access_token: 'fixture-token', expires_in: 3600 }));
      }
      if (init.method === 'GET') {
        return new Response(JSON.stringify({ ai_profiles: [getResponse] }));
      }
      writes.push({ method: String(init.method), body: JSON.parse(String(init.body)) });
      return new Response(JSON.stringify(fixture));
    });
    vi.stubGlobal('fetch', fetch);
    const mgmt = client();
    // This replay uses the complete response as input, not an inferred missing request tail.
    expect(await mgmt.profiles.create(fixture)).toEqual(fixture);
    const found = await mgmt.profiles.get(fixture.profile_id);
    expect(found).toEqual(getResponse);
    expect(await mgmt.profiles.update(fixture.profile_id, found)).toEqual(fixture);
    expect(writes).toEqual([
      { method: 'POST', body: fixture },
      { method: 'PUT', body: getResponse },
    ]);
    expect(fetch.mock.calls.filter(([url]) => String(url).includes('/oauth2/token'))).toHaveLength(
      1,
    );
  });
});
