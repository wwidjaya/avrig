# Blender Bulb authoring scaffold

This folder provides a repeatable starting point for authoring the approved Bulb PET in Blender 5.2 and exporting it for the Bevy viewer.

## What the script does

[setup_bulb_scene.py](setup_bulb_scene.py) adds the following without deleting existing scene content:

- `PET_Character/Geometry` and `PET_Character/Rig` collections for the model and armature.
- A preview camera and three area lights in a separate `PET_Preview_Stage` collection.
- Metric units, a 30 FPS timeline, a 60-frame preview range, and a square render resolution.

It does not create character geometry, rig bones, or animation. The approved concept artwork remains the visual reference. The looping idle action should be named `Idle`; include it in the glTF export so the Bevy viewer can play it.

## Run in Blender

1. Open Blender 5.2 and switch to the **Scripting** workspace.
2. Open `setup_bulb_scene.py` in the Text Editor and press **Run Script**.
3. Model Bulb under `PET_Character/Geometry` and place its armature under `PET_Character/Rig`.
4. Create and test the rigged animation actions, starting with a subtle looping `Idle`.
5. Save the `.blend` source file.

The script does not clear the current scene. If you want a clean file, start from Blender's General template before running it.

## Export a GLB

With the Bulb model and rig in the `PET_Character` collection and the glTF 2.0 exporter enabled, run this in Blender's Python Console, replacing the output path with your local repository path:

```python
import bpy
from pathlib import Path

character = bpy.data.collections["PET_Character"]
objects = list(character.all_objects)
meshes = [obj for obj in objects if obj.type == "MESH"]
armatures = [obj for obj in objects if obj.type == "ARMATURE"]
if not meshes or not armatures:
    raise RuntimeError("PET_Character needs a mesh and an armature before export")

bpy.ops.object.select_all(action="DESELECT")
for obj in objects:
    obj.select_set(True)
bpy.context.view_layer.objects.active = armatures[0]
bpy.ops.export_scene.gltf(
    filepath=str(Path(r"D:\path\to\avrig-ai\avatar\viewer\assets\avatars\bulb.glb")),
    export_format="GLB",
    use_selection=True,
    export_animations=True,
)
```

The same script can set up a scene and export in background mode when passed a saved `.blend` file:

```powershell
blender.exe "D:\path\to\bulb.blend" --background --python "D:\path\to\avrig-ai\avatar\blender\setup_bulb_scene.py" -- --export-glb "D:\path\to\avrig-ai\avatar\viewer\assets\avatars\bulb.glb"
```

Use the model path configured for the viewer if you choose a different GLB location. Check the exported file in Blender and in the Bevy viewer; verify the silhouette, materials, skin deformation, and animation loop. The viewer expects the first exported animation to be the idle clip, so confirm the GLB animation order before treating the export as accepted.

## Current limits

- This is setup/export scaffolding, not a finished PET asset or automated character generator.
- Rigging, animation authoring, visual review, and export-order verification remain manual.
- Blender is not available in the repository editing environment used to add these files, so the script has not been executed here.
