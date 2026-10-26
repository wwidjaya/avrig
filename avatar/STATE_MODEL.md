# Avatar State and Presentation Contract

**Status:** Proposed implementation detail for the Avatar Development Plan.

This document makes the Avatar plan’s state behavior explicit. The Avatar is a presentation layer: it displays validated status supplied by the client. It does not establish connectivity, infer activity, authenticate users, or own employee identity.

## State dimensions

Connection and activity are independent. Motion preference controls how transitions are rendered, not what the state means.

### Connection state

| State | Outfit | Visible and accessible meaning |
|---|---|---|
| not_connected | T-shirt-and-jeans base outfit | PET is not connected |
| connecting | T-shirt-and-jeans base outfit | A connection attempt is in progress |
| connected | Same T-shirt-and-jeans outfit plus a casual jacket and shared chest badge | Connection has been confirmed |
| connection_lost | T-shirt-and-jeans base outfit after loss is confirmed | The established connection was lost |
| unknown | T-shirt-and-jeans base outfit | Connection state is unknown |

Only a current, affirmatively confirmed connected state may show the jacket and connected badge. These are connection cues only; they do not indicate authority, role, activity, or trust.

### Activity state

| State | Presentation guidance | Example accessible label suffix |
|---|---|---|
| idle | Subtle breathing and blinking | idle |
| listening | Attentive pose or brief bounded cue | listening |
| thinking | Short, low-motion thinking cue | thinking |
| working | Focused pose or small bounded cue | working |
| waiting_for_user | Neutral expectant pose | waiting for you |
| message_alert | Present a message/alert with a compact tablet prop and a clear, non-alarming cue | new message or alert |
| error | Clear, non-alarming cue | needs attention |

Activity changes never change connection clothing. A connected PET that is thinking has the connected jacket and chest badge; a disconnected PET that is thinking stays in its character-appropriate base outfit. The message/alert tablet may appear in either connection state. Use the same simple activity-motion language across the collection. Message/alert may show the PET holding a compact tablet; the tablet is not a connection cue, and message contents must not be exposed in artwork.

### Motion preference

- full: allow approved short transitions and bounded activity cues.
- reduced: simplify or shorten transitions; avoid repeated motion.
- static: use static presentation while keeping the same semantic state and label.

Respect the operating-system reduced-motion setting when available. User preference may reduce motion further.

## Transition rules

Expected connection flow:

    not_connected → connecting → connected
                               ↓
                       connection_lost → connecting
                               ↓
                         not_connected

- not_connected to connecting: retain base outfit; show connecting label/cue.
- connecting to connected: add the form-appropriate jacket and shared chest badge only after the client confirms current connectivity.
- connected to connection_lost: client applies a configurable grace period for transient disruptions; when loss is confirmed, remove the connected jacket and show loss.
- connection_lost to connecting: retain base outfit and show reconnecting.
- Any state to unknown or stale: remove the connected jacket and badge unless connected is affirmatively current.
- Activity transitions do not change connection state or jacket/badge. Message/alert may add or remove the tablet prop.
- State text updates immediately and must not wait for an animation to finish.
- Transitions are interruptible and respect reduced/static motion modes.

The Edge/client owns connectivity truth and timing. The Avatar consumes the resolved state; the jacket or badge must never be used as evidence that a service is connected.

## Presentation input and resolved output

Input to the presentation runtime:

- Stable avatar ID and asset version/manifest.
- Connection state, freshness indicator, and update sequence or timestamp.
- Activity state, including message/alert presentation.
- A validated, privacy-safe notification summary or presence flag when message/alert is shown; do not pass message content to the avatar renderer.
- Surface size, theme, locale, and motion preference.
- Optional visibility/active-surface hint.

Resolved output:

- Compatible asset set, outfit, and pose.
- Optional bounded animation.
- Accessible name and status label.
- Stale indicator and fallback diagnostic where relevant.

Do not include credentials, private prompts, task contents, or sensitive Office data in avatar definitions, labels, or asset manifests.

## Safe fallback and ordering

- Missing, unknown, or stale connection data resolves to the character-appropriate base outfit with an unknown/disconnected label.
- Unknown activity resolves to a neutral pose and generic label; do not invent an activity.
- Message/alert presentation must not reveal message text, sender identity, or sensitive content in avatar artwork.
- Unknown or retired avatar ID resolves to an approved default if one exists, otherwise a neutral fallback. Preserve the stored ID so the preference can be recovered.
- Missing, corrupt, or incompatible artwork falls back to a bundled neutral asset and retains the correct text state.
- Reject invalid state values to a safe fallback and record a diagnostic.
- An old asynchronous update must not overwrite a newer state. Use a defined sequence-number or timestamp ordering rule and verify it.

## Accessibility requirements

- Every connection/activity combination has an accurate text equivalent, for example “PET connected, thinking.”
- Message/alert has a clear accessible label; the tablet itself is not the sole signal, and the artwork does not expose message contents.
- State is not conveyed by color or motion alone.
- The jacket and shared chest badge remain distinguishable at target sizes; the text label remains available if they do not.
- Respect reduced-motion settings and provide a static mode.
- Decorative elements are hidden from assistive technology where appropriate; meaningful status is exposed through the accessibility tree.
- Labels describe the state without implying human presence or authority.

## Asset compatibility

Each asset is versioned and keyed by stable avatar ID, connection outfit, activity pose, scale, and theme as applicable. The base outfit and connected outer-layer/badge variant must match in identity, palette, pose, and version. Prefer a shared base plus jacket layer; use paired composite variants if layering causes seams or visual mismatch.

The manifest is data-only and records dimensions, format, checksum, source reference, author, license, provenance, accessible name/descriptions, supported surfaces, and minimum renderer version. Do not load executable content from a manifest. Verify provenance/license and integrity before release.

## Runtime boundary

    Validated client status
             ↓
    Input validation and freshness check
             ↓
    State resolver
       ├── outfit resolver
       ├── pose/animation selector
       └── accessibility label builder
             ↓
    Versioned asset loader/cache
             ↓
    Surface renderer and status text

Start with a local preview/state selector. The avatar renderer has no service credentials and does not call Office Service. A later typed Edge adapter may supply authorized status. Office data remains minimized and is fetched/filtered by Edge.

## Verification cases

For every released avatar, verify:

- All connection states map to the specified outfit and label.
- Only current connected state shows the connected jacket and badge.
- Every activity works independently of connection outfit; message/alert may show the tablet in connected and disconnected states.
- Unknown, stale, invalid, out-of-order, and missing inputs fall back safely.
- Reduced-motion and static modes preserve semantic meaning.
- State text is available without color or animation.
- Missing/corrupt/incompatible assets trigger fallback.
- Base and connected outer-layer assets match identity and version.
- Labels and metadata expose no secrets or sensitive task content.
- Avatar code cannot mutate Office state or determine authorization.
