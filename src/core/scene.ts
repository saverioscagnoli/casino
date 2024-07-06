import * as THREE from "three";
import { Camera } from "~/core/camera";
import { Renderer } from "~/core/renderer";
import { AssetDatabase } from "./asset-database";
import { Table } from "~/game/table";
import { CustomModel } from "~/lib/enums";

class Scene extends THREE.Scene {
  private static instance: Scene;

  // prettier-ignore
  public static screenResolution = new THREE.Vector2(window.innerWidth, window.innerHeight);
  public static aspectRatio = Scene.screenResolution.x / Scene.screenResolution.y;

  private static textureLoader = new THREE.TextureLoader();

  private camera: Camera;
  private renderer: Renderer;
  private table: Table;

  private constructor() {
    super();

    this.camera = new Camera();
    this.renderer = new Renderer();
    this.table = new Table();
  }

  public static build(): Scene {
    if (!Scene.instance) {
      Scene.instance = new Scene();
    }

    return Scene.instance;
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

  private importDecoration(model: CustomModel, pos: THREE.Vector3, scale: THREE.Vector3) {
    const db = AssetDatabase.build();

    const deco = db.getModel(model);

    deco.scene.position.copy(pos);
    deco.scene.scale.copy(scale);

    deco.scene.castShadow = true;
    deco.scene.receiveShadow = true;

    deco.scene.traverse(child => {
      if (child instanceof THREE.Mesh) {
        child.castShadow = true;
        child.receiveShadow = true;
      }
    });

    this.add(deco.scene);

    return deco;
  }

  private createLights() {
    const directionalLight = new THREE.DirectionalLight(0xffffff, 2);
    directionalLight.position.set(0, 20, 0);

    directionalLight.castShadow = true;

    this.add(directionalLight);

    // const ambientLight = new THREE.AmbientLight(0xffffff, 0.5);

    // this.add(ambientLight);

    const spotLight = new THREE.SpotLight(0xffffff, 10, 100, Math.PI, 0.5, 1);
    spotLight.position.set(0, 10, 0);

    spotLight.castShadow = true;

    this.add(spotLight);
  }

  private createFloor() {
    const db = AssetDatabase.build();

    const texture = db.getTexture("carpet");

    const floorGeometry = new THREE.PlaneGeometry(150, 100);
    const floorMaterial = new THREE.MeshStandardMaterial({
      color: 0xff0000,
      side: THREE.DoubleSide
    });

    texture.wrapS = THREE.RepeatWrapping;
    texture.wrapT = THREE.RepeatWrapping;

    texture.repeat.set(5, 5);

    const floor = new THREE.Mesh(floorGeometry, floorMaterial);

    floor.position.set(0, -1, 0);
    floor.rotation.x = Math.PI / 2;

    floor.receiveShadow = true;

    this.add(floor);
  }

  private createDecorations() {
    // Pool table behind the blackjack table
    this.importDecoration(
      CustomModel.PoolTable,
      new THREE.Vector3(10, 0, -20),
      new THREE.Vector3(1, 1, 1)
    );

    // Pinball in the back
    this.importDecoration(
      CustomModel.Pinball,
      new THREE.Vector3(25, 0, -20),
      new THREE.Vector3(3, 3, 3)
    );

    const arcadeMachine = this.importDecoration(
      CustomModel.ArcadeMachine,
      new THREE.Vector3(-18, 0, -25),
      new THREE.Vector3(1, 1, 1)
    );

    arcadeMachine.scene.rotateY(Math.PI / 2);

    const clones = [...Array(5)].map(() => arcadeMachine.scene.clone());

    for (const [i, clone] of clones.entries()) {
      clone.position.set(-3 - i * 3, 0, -25);

      this.add(clone);
    }

    const airHockey = this.importDecoration(
      CustomModel.AirHockey,
      new THREE.Vector3(-30, 0, -25),
      new THREE.Vector3(1, 1, 1)
    );

    airHockey.scene.rotateY(-Math.PI / 2);

    const airHockeyClone = airHockey.scene.clone();
    airHockeyClone.position.set(-30, 0, -18);

    this.add(airHockeyClone);

    const jukebox = this.importDecoration(
      CustomModel.Jukebox,
      new THREE.Vector3(30, 1, -13),
      new THREE.Vector3(3, 3, 3)
    );

    jukebox.scene.rotateY(Math.PI / 2);
  }

  /**
   * This function will run only once
   * when the scene is loaded.
   */
  public async load() {
    // This should be the first time the AssetDatabase is being used
    const db = AssetDatabase.build();

    // Load all the models and textures
    await db.fetchCards();
    await db.fetchTextures();
    await db.fetchModels();

    // Initialize the camera and renderer
    this.camera.init(this, this.renderer);
    this.renderer.init(this);

    // Create the table
    this.table.init(this, 5);

    // Create the floor
    this.createFloor();

    // Create the decorations inside the scene
    this.createDecorations();

    // Create the lights
    this.createLights();

    const cubeGeometry = new THREE.BoxGeometry(3, 3, 3);
    const cubeMaterial = new THREE.MeshPhongMaterial({ color: 0xff0000 });
    const cube = new THREE.Mesh(cubeGeometry, cubeMaterial);

    cube.castShadow = true;

    cube.position.set(-3, 4, -20);

    this.add(cube);
  }

  public update() {
    this.camera.update();
    this.table.update();
  }

  public render() {
    this.renderer.renderComposer();
  }
}

export { Scene };

// public async createPoolTable() {
//   const poolTableSrc = "/src/assets/pool-table/scene.gltf";

//   const poolTable = await this.GLTFLoader.loadAsync(poolTableSrc);

//   poolTable.scene.position.set(10, 0, -20);
//   poolTable.scene.castShadow = true;
//   poolTable.scene.receiveShadow = true;

//   this.add(poolTable.scene);
// }

// public async createSlotMachine() {
//   const slotMachineSrc = "/src/assets/slot/scene.gltf";

//   const slotMachine = await this.GLTFLoader.loadAsync(slotMachineSrc);

//   slotMachine.scene.position.set(-10, 2, -37);
//   slotMachine.scene.castShadow = true;
//   slotMachine.scene.receiveShadow = true;

//   slotMachine.scene.scale.set(10, 10, 10);
//   slotMachine.scene.rotation.y = -Math.PI / 2;

//   this.add(slotMachine.scene);
// }

// public async createPokerTable() {
//   const pokerTableSrc = "/src/assets/poker-table/model.glb";

//   const pokerTable = await this.GLTFLoader.loadAsync(pokerTableSrc);

//   pokerTable.scene.position.set(-12, -1, -10);
//   pokerTable.scene.castShadow = true;
//   pokerTable.scene.receiveShadow = true;

//   pokerTable.scene.scale.set(3, 3, 3);
//   pokerTable.scene.rotation.y = Math.PI;

//   const pointLight = new THREE.PointLight(0xffffff, 500, 100);

//   pointLight.position.set(-12, 10, -10);

//   this.add(pointLight);
//   this.add(pokerTable.scene);
// }
