import * as THREE from "three";
import { OrbitControls } from "three/examples/jsm/controls/OrbitControls.js";
import { Scene } from "~/core/scene";
import { Renderer } from "~/core/renderer";

class Camera extends THREE.PerspectiveCamera {
  public static FOV = 75;
  public static NEAR = 0.1;
  public static FAR = 1000;

  private controls!: OrbitControls;

  public constructor() {
    super(Camera.FOV, Scene.aspectRatio, Camera.NEAR, Camera.FAR);
  }

  public init(scene: Scene, renderer: Renderer) {
    this.position.set(0, 12, 10);
    this.lookAt(scene.position);
    this.controls = new OrbitControls(this, renderer.domElement);

    // this.controls.enableDamping = true;
    // this.controls.dampingFactor = 0.1;

    // this.controls.minDistance = 10;
    // this.controls.maxDistance = 15;

    // this.controls.enableRotate = false;
    // this.controls.screenSpacePanning = false; // Disable screen space panning

    // this.controls.addEventListener("change", () => {
    //   this.controls.target.z = 0;
    //   this.position.z = 10;
    // });
  }

  public update() {
    this.controls.update();

    // Clamp the camera's x position to prevent it from moving past x = -5
    // if (this.position.x < -5) {
    //   this.position.x = -5;
    // } else if (this.position.x > 5) {
    //   this.position.x = 5;
    // }

    // if (this.controls.target.x < -5) {
    //   this.controls.target.x = -5;
    // } else if (this.controls.target.x > 5) {
    //   this.controls.target.x = 5;
    // }
  }
}

export { Camera };
