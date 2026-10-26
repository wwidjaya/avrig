"""Set up a non-destructive Blender 5.2 workspace for the PET Bulb asset.

Run from Blender's Scripting workspace. The script creates/links standard
collections, scene timing, a camera, and preview lights without deleting scene
objects. It does not generate the final character or fake animation.
"""

from pathlib import Path

import bpy
from mathutils import Vector


CHARACTER_COLLECTION = "PET_Character"
GEOMETRY_COLLECTION = "Geometry"
RIG_COLLECTION = "Rig"
STAGE_COLLECTION = "PET_Preview_Stage"
CAMERA_NAME = "PET_Preview_Camera"
IDLE_ACTION_NAME = "Idle"


def ensure_collection(name: str, parent=None):
    """Get a collection by name and link it under parent if needed."""
    collection = bpy.data.collections.get(name)
    if collection is None:
        collection = bpy.data.collections.new(name)

    owner = parent or bpy.context.scene.collection
    if collection.name not in owner.children:
        owner.children.link(collection)
    return collection


def aim_at(obj, target):
    direction = Vector(target) - obj.location
    obj.rotation_euler = direction.to_track_quat("-Z", "Y").to_euler()


def ensure_camera(stage):
    camera_obj = bpy.data.objects.get(CAMERA_NAME)
    if camera_obj is None:
        camera_data = bpy.data.cameras.new(CAMERA_NAME)
        camera_obj = bpy.data.objects.new(CAMERA_NAME, camera_data)
        stage.objects.link(camera_obj)

    camera_obj.location = (4.2, -6.5, 2.6)
    camera_obj.data.lens = 52
    aim_at(camera_obj, (0.0, 0.0, 1.0))
    bpy.context.scene.camera = camera_obj
    return camera_obj


def ensure_area_light(stage, name, location, energy, size, color):
    light_obj = bpy.data.objects.get(name)
    if light_obj is None:
        light_data = bpy.data.lights.new(name=name, type="AREA")
        light_obj = bpy.data.objects.new(name, light_data)
        stage.objects.link(light_obj)

    light_obj.location = location
    light_obj.data.energy = energy
    light_obj.data.shape = "DISK"
    light_obj.data.size = size
    light_obj.data.color = color
    aim_at(light_obj, (0.0, 0.0, 1.0))
    return light_obj


def setup_scene():
    scene = bpy.context.scene

    character = ensure_collection(CHARACTER_COLLECTION)
    ensure_collection(GEOMETRY_COLLECTION, character)
    ensure_collection(RIG_COLLECTION, character)

    stage = ensure_collection(STAGE_COLLECTION)
    ensure_camera(stage)
    ensure_area_light(
        stage, "PET_Key_Light", (3.5, -4.0, 5.0), 450, 4.0, (1.0, 0.82, 0.62)
    )
    ensure_area_light(
        stage, "PET_Fill_Light", (-4.0, -2.0, 2.8), 260, 3.5, (0.58, 0.72, 1.0)
    )
    ensure_area_light(
        stage, "PET_Rim_Light", (0.5, 3.5, 4.0), 500, 3.0, (1.0, 0.94, 0.78)
    )

    scene.unit_settings.system = "METRIC"
    scene.render.fps = 30
    scene.frame_start = 1
    scene.frame_end = 60
    scene.render.resolution_x = 900
    scene.render.resolution_y = 900
    scene.render.resolution_percentage = 100
    scene.frame_set(1)

    print("PET Bulb scene scaffold is ready.")
    print(f"Put avatar meshes in {CHARACTER_COLLECTION}/{GEOMETRY_COLLECTION}.")
    print(f"Put the armature in {CHARACTER_COLLECTION}/{RIG_COLLECTION}.")
    print(f"Name the looping idle action '{IDLE_ACTION_NAME}' before export.")
    return character


def export_character_glb(character_collection, output_path):
    """Export only objects in the PET character collection, excluding preview stage."""
    character_objects = list(character_collection.all_objects)
    meshes = [obj for obj in character_objects if obj.type == "MESH"]
    armatures = [obj for obj in character_objects if obj.type == "ARMATURE"]

    if not meshes:
        raise RuntimeError(
            f"No mesh found under {CHARACTER_COLLECTION}; build the character first."
        )
    if not armatures:
        raise RuntimeError(
            f"No armature found under {CHARACTER_COLLECTION}; rig the character first."
        )

    output = Path(output_path).expanduser().resolve()
    output.parent.mkdir(parents=True, exist_ok=True)

    bpy.ops.object.select_all(action="DESELECT")
    for obj in character_objects:
        obj.select_set(True)
    bpy.context.view_layer.objects.active = armatures[0]

    bpy.ops.export_scene.gltf(
        filepath=str(output),
        export_format="GLB",
        use_selection=True,
        export_animations=True,
    )
    print(f"Exported PET GLB: {output}")


def command_line_export_path():
    """Read an optional --export-glb <path> after Blender's -- separator."""
    import sys

    args = sys.argv
    if "--" not in args:
        return None
    user_args = args[args.index("--") + 1 :]
    if "--export-glb" not in user_args:
        return None
    option_index = user_args.index("--export-glb")
    if option_index + 1 >= len(user_args):
        raise RuntimeError("--export-glb requires an output path.")
    return user_args[option_index + 1]


def main():
    character = setup_scene()
    export_path = command_line_export_path()
    if export_path:
        export_character_glb(character, export_path)


if __name__ == "__main__":
    main()
