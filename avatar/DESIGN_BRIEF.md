# Avatar Visual Design Brief

**Status:** User-approved direction captured 2026-10-26. Individual model sheets and production assets still require visual review.

## Collection direction

The PET catalogue is a cohesive collection of **polished, expressive, stylized 3D chibi characters** with an office-casual presentation. Each character has a **distinct silhouette** and a **bright, saturated palette unique to that avatar**. Materials may vary to suit each concept while sharing consistent quality, proportions, lighting, and finish. Faces use large expressive eyes and simple features.

Each avatar’s character form directly reflects its name and concept. Characters may carry a compact tablet when presenting a message or alert; the tablet is a presentation prop and never communicates connection state. The visual cues below are starting art directions for design sheets, not finished model specifications.

| ID | Name | Starting character-form direction |
|---|---|---|
| AV-01 | Bulb | Rounded bulb-shaped character with a small luminous detail |
| AV-02 | Nova | Star or starburst-inspired explorer |
| AV-03 | Draco | Friendly chibi dragon with restrained scale or horn details |
| AV-04 | Sage | Calm, wise guide with leaf or book-inspired form details |
| AV-05 | Foxy | Playful fox character with a clear fox-like silhouette |
| AV-06 | Lumi | Soft luminous character with a gentle light motif |
| AV-07 | Bamboo | Grounded character with bamboo-inspired shape or markings |
| AV-08 | Orbit | Curious space character with an orbital-ring motif |
| AV-09 | Leafy | Nature-positive leaf-shaped or leaf-creature character |
| AV-10 | Cloudy | Gentle character with a cloud-like silhouette |
| AV-11 | Pixel | Digital character with a crisp pixel-grid form |
| AV-12 | Aura | Expressive light character with a soft halo form |
| AV-13 | Rocky | Steady character with a rounded stone-like body |
| AV-14 | Spark | Energetic character with a spark-inspired silhouette |
| AV-15 | Aqua | Fluid character with wave or droplet forms |
| AV-16 | Byte | Compact technical character with modular geometric forms |
| AV-17 | Zen | Quiet, balanced character built around a minimal circular motif |
| AV-18 | Comet | Adventurous character with a comet-tail silhouette |
| AV-19 | Ivy | Resourceful growing character with vine or leaf forms |
| AV-20 | Neo | Modern generalist with a clean geometric silhouette |
| AV-21 | Alex | Professional male office worker with a friendly human chibi silhouette and restrained office-casual details |
| AV-22 | Maya | Professional female office worker with a friendly human chibi silhouette and restrained office-casual details |

## Outfit and connection state

Use form-appropriate clothing while preserving the established visual meaning:

- **Not connected:** each character has its own base outfit, adapted to its body and concept.
- **Connected:** add a form-appropriate outer layer and the same small chest badge across the collection.
- The chest badge is a shared connection cue. Its states must also use distinct shapes/icons and text labels; color alone is insufficient.
- Outer layer and badge communicate connection only. They do not imply authority, role, activity, or trust.
- Activity does not change connection clothing. A message/alert presentation may add a compact tablet prop.

The earlier human outfit language (T-shirt and jeans, with an added jacket when connected) is adapted for non-human and unusual forms rather than imposed literally on every model.

## Framing and animation

- The floating desktop PET uses a full-body view.
- Idle behavior is subtle breathing and blinking, with no distracting continuous motion.
- Listening, thinking, working, waiting, message/alert, and error use the same simple motion language across all avatars. Character-specific motion is not required for the first collection.
- Reuse animation rigs where the character forms allow it; use compatible tailored rigs when a distinct silhouette requires them.
- Reduced-motion and static modes preserve all state information.

## Catalogue and selection

All 20 avatars should be available in the first catalogue release. Bulb is the initial default; users can change their choice later. Stable IDs remain attached to the concepts even if names or artwork change.

## Implementation implications

1. Develop Bulb first as the reference model to validate the collection style, outfit-state adaptation, chest badge, full-body desktop framing, and shared animation language.
2. Produce a low-fidelity form sheet for all 20 before final modeling to confirm silhouette variety and shared collection coherence.
3. Build and validate reusable rigs by character archetype; do not force a shared rig where it damages a form.
4. Once Bulb and the shared style are approved, create the remaining 19 models and state variants before the first catalogue release.
5. Keep the avatar renderer independent from Employee identity, permissions, agent behavior, and Office data. A later typed client interface may supply state.

## Still to decide during visual review

- Exact material and surface treatment for each concept.
- Final palette values and contrast requirements for every avatar.
- Exact design of the shared chest badge for each connection state.
- Detailed model sheets, silhouette, and expression range for each concept.
- Supported desktop size/resolution targets and performance budgets.
