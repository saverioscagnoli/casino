import * as THREE from "three";
import { EffectComposer, RenderPixelatedPass } from "three/examples/jsm/Addons.js";
import { Scene } from "~/core/scene";

class Renderer extends THREE.WebGLRenderer {
  public static PIXEL_SIZE: number = 0.3;
  private composer: EffectComposer;

  public constructor() {
    super({ antialias: false });

    this.composer = new EffectComposer(this);
  }

  public init(scene: Scene) {
    this.setSize(Scene.screenResolution.x, Scene.screenResolution.y);
    this.setPixelRatio(window.devicePixelRatio);

    this.shadowMap.enabled = true;
    this.shadowMap.type = THREE.PCFSoftShadowMap;

    this.composer.renderer.shadowMap.enabled = true;

    document.body.appendChild(this.domElement);

    const renderPass = new RenderPixelatedPass(
      Renderer.PIXEL_SIZE,
      scene,
      scene.getCamera()
    );

    this.composer.addPass(renderPass);
  }

  public renderComposer() {
    this.composer.render();
  }
}

export { Renderer };
