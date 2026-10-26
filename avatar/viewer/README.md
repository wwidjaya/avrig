# Bulb model review viewer

This is a small native Bevy viewer for reviewing the real PET model and its exported animation. It intentionally does not generate a replacement character from primitive shapes.

## Model input

Place the reviewed, rigged GLB here:

```text
avatar/viewer/assets/avatars/bulb.glb
```

Or set `AVRIG_AVATAR_MODEL` to a path relative to `avatar/viewer/assets/`.

The first animation in the GLB (animation index `0`) is played in a loop as the idle animation. Export the idle clip first. If the model has no GLB animation, Bevy will report that it could not load the animation; the viewer still provides a small root float for checking the model in motion.

## Controls

- Left mouse drag: orbit around the model.
- Mouse wheel: zoom.
- `R`: reset the camera.
- `Space`: toggle slow automatic rotation.

## Run

From this directory:

```sh
cargo run
```

The viewer is a model review tool, not the Tauri desktop PET or the Office UI. Keep the avatar renderer and its state contract separate from Employee identity and Office service data.

## Model review checklist

Before treating the GLB as the Bulb reference asset, compare its front, side, and rear views with [the approved model sheet](../assets/concepts/bulb-model-sheet-v1.png). In particular, confirm the glowing glass bulb silhouette and filament, the compact connector, chibi proportions, expressive embedded eyes, white T-shirt, blue jeans, hands-on-hips pose, and white sneakers. The connected jacket/badge and tablet should remain separate reviewable states.
