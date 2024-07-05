import * as THREE from "three";
import { GLTFLoader } from "three/examples/jsm/Addons.js";
import { Camera } from "~/core/camera";
import { Renderer } from "~/core/renderer";
import { Deck } from "~/core/deck";

import flipCardAudioSrc from "~/assets/sounds/card-flip.mp3";

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

  private deck: Deck;

  public constructor() {
    super();

    this.camera = new Camera();
    this.renderer = new Renderer();

    this.GLTFLoader = new GLTFLoader();

    this.camera.init(this);
    this.renderer.init(this);
    this.deck = null as any;
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

  public getDeck(): Deck {
    return this.deck;
  }

  public createLights() {
    const ambientLight = new THREE.AmbientLight(0xffffff, 2);
    this.add(ambientLight);
  }

  public createTable() {
    const tableGeometry = new THREE.PlaneGeometry(40, 20);
    const tableMaterial = new THREE.MeshBasicMaterial({
      color: Scene.TABLE_COLOR,
      side: THREE.DoubleSide
    });
    const tableMesh = new THREE.Mesh(tableGeometry, tableMaterial);

    tableMesh.rotation.x = -Math.PI / 2;
    this.add(tableMesh);
  }

  public async createPoolTable() {
    const poolTableSrc = "/src/assets/pool-table/scene.gltf";

    const poolTable = await this.GLTFLoader.loadAsync(poolTableSrc);

    poolTable.scene.position.set(10, 0, -20);
    poolTable.scene.castShadow = true;
    poolTable.scene.receiveShadow = true;

    this.add(poolTable.scene);
  }

  public async load() {
    await this.createPoolTable();

    this.createLights();
    this.createTable();

    this.deck = await Deck.create();
    this.deck.shuffle();

    for (const [i, card] of this.deck.iter()) {
      card.getMesh().position.set(12 + i * 0.1, 1, -6);

      card.getMesh().rotation.x = -Math.PI / 2;
      card.getMesh().rotation.y = Math.PI / 2;

      this.add(card.getMesh());
    }
  }

  public update() {
    this.deck.update();
  }

  public render() {
    this.renderer.renderComposer();
  }
}

export { Scene };
