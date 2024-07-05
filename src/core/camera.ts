import * as THREE from "three";
import { Scene } from "~/core/scene";

class Camera extends THREE.PerspectiveCamera {
  public static FOV = 75;
  public static NEAR = 0.1;
  public static FAR = 1000;

  public constructor() {
    super(
      Camera.FOV,
      Scene.aspectRatio,
      Camera.NEAR,
      Camera.FAR
    );
  }

  public init(scene: Scene) {
    this.position.set(0, 10, 10);
    this.lookAt(scene.position);
  }
}

export { Camera };
