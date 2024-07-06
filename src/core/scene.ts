import * as THREE from "three";
import { GLTFLoader } from "three/examples/jsm/Addons.js";
import { Camera } from "~/core/camera";
import { Renderer } from "~/core/renderer";
import { Deck } from "~/core/deck";

import flipCardAudioSrc from "~/assets/sounds/card-flip.mp3";
import { Table } from "~/core/table";

class Scene extends THREE.Scene {
  // prettier-ignore
  public static screenResolution = new THREE.Vector2(window.innerWidth, window.innerHeight);
  public static aspectRatio = Scene.screenResolution.x / Scene.screenResolution.y;
  public static TABLE_COLOR = 0x4e9164;
  public static flipCardAudio = new Audio(flipCardAudioSrc);

  private static textureLoader = new THREE.TextureLoader();

  private camera: Camera;
  private renderer: Renderer;
  private GLTFLoader: GLTFLoader;

  private table!: Table;

  public constructor() {
    super();

    this.camera = new Camera();
    this.renderer = new Renderer();

    this.GLTFLoader = new GLTFLoader();

    this.camera.init(this);
    this.renderer.init(this);
  }

  public static loadTexture(src: string): Promise<THREE.Texture> {
    return Scene.textureLoader.loadAsync(src);
  }

  public getCamera(): Camera {
    return this.camera;
  }

  public getRenderer(): Renderer {
    return this.renderer;
  }

  public createLights() {
    const ambientLight = new THREE.AmbientLight(0xffffff, 2);

    this.add(ambientLight);
  }

  public async createPoolTable() {
    const poolTableSrc = "/src/assets/pool-table/scene.gltf";

    const poolTable = await this.GLTFLoader.loadAsync(poolTableSrc);

    poolTable.scene.position.set(10, 0, -20);
    poolTable.scene.castShadow = true;
    poolTable.scene.receiveShadow = true;

    this.add(poolTable.scene);
  }

  public async createSlotMachine() {
    const slotMachineSrc = "/src/assets/slot/scene.gltf";

    const slotMachine = await this.GLTFLoader.loadAsync(slotMachineSrc);

    slotMachine.scene.position.set(-10, 2, -37);
    slotMachine.scene.castShadow = true;
    slotMachine.scene.receiveShadow = true;

    slotMachine.scene.scale.set(10, 10, 10);
    slotMachine.scene.rotation.y = -Math.PI / 2;

    this.add(slotMachine.scene);
  }

  public async createPokerTable() {
    const pokerTableSrc = "/src/assets/poker-table/model.glb";

    const pokerTable = await this.GLTFLoader.loadAsync(pokerTableSrc);

    pokerTable.scene.position.set(-12, -1, -10);
    pokerTable.scene.castShadow = true;
    pokerTable.scene.receiveShadow = true;

    pokerTable.scene.scale.set(3, 3, 3);
    pokerTable.scene.rotation.y = Math.PI;

    const pointLight = new THREE.PointLight(0xffffff, 500, 100);

    pointLight.position.set(-12, 10, -10);

    this.add(pointLight);
    this.add(pokerTable.scene);
  }

  /**
   * This function will run only once
   * when the scene is loaded.
   */
  public async load() {
    this.table = await Table.create();

    await this.createPoolTable();
    await this.createSlotMachine();
    await this.createPokerTable();

    this.createLights();
    this.table.init(this, 5);
  }

  public update() {
    this.table.update();
  }

  public render() {
    this.renderer.renderComposer();
  }
}

export { Scene };
