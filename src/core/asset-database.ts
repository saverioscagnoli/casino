import * as THREE from "three";
import { GLTF, GLTFLoader } from "three/examples/jsm/Addons.js";
import { createLabelsArray, createSuitsArray, getCardName } from "~/lib/utils";
import { CustomModel } from "~/lib/enums";

// @ts-ignore
import pokerTableSrc from "~/assets/poker-table/model.glb";
// @ts-ignore
import slotMachineSrc from "~/assets/slot/scene.gltf";
// @ts-ignore
import poolTableSrc from "~/assets/pool-table/scene.gltf";
// @ts-ignore
import pinballSrc from "~/assets/pinball/scene.gltf";
// @ts-ignore
import arcadeMachineSrc from "~/assets/arcade-machine/scene.glb";
// @ts-ignore
import airHockeySrc from "~/assets/air-hockey/scene.glb";
// @ts-ignore
import jukeboxSrc from "~/assets/jukebox/scene.glb";

import tableTexture from "~/assets/table.png";
import carpetTexture from "~/assets/carpet.jpg";

const CARD_FOLDER = "/src/assets/cards";

class AssetDatabase {
  private static instance: AssetDatabase;

  private textures: Map<string, THREE.Texture>;
  private models: Map<CustomModel, GLTF>;

  private textureLoader: THREE.TextureLoader;
  private gltfLoader: GLTFLoader;

  private constructor() {
    this.models = new Map();
    this.textures = new Map();

    this.textureLoader = new THREE.TextureLoader();
    this.gltfLoader = new GLTFLoader();
  }

  // Singleton pattern
  public static build(): AssetDatabase {
    if (!AssetDatabase.instance) {
      AssetDatabase.instance = new AssetDatabase();
    }

    return AssetDatabase.instance;
  }

  private loadTexture(src: string): Promise<THREE.Texture> {
    return this.textureLoader.loadAsync(src);
  }

  private loadModel(src: string): Promise<GLTF> {
    return this.gltfLoader.loadAsync(src);
  }

  public async fetchCards() {
    const suits = createSuitsArray();
    const labels = createLabelsArray();
    const promises: { name: string; texture: Promise<THREE.Texture> }[] = [];

    for (const s of suits) {
      for (const l of labels) {
        const cardName = getCardName(s, l);
        const textureUrl = `${CARD_FOLDER}/${cardName}.png`;

        promises.push({
          name: cardName,
          texture: this.loadTexture(textureUrl)
        });
      }
    }

    const redBack = this.loadTexture(`${CARD_FOLDER}/back-red.png`);
    const blueBack = this.loadTexture(`${CARD_FOLDER}/back-blue.png`);

    promises.push({ name: "back-red", texture: redBack });
    promises.push({ name: "back-blue", texture: blueBack });

    const resolved = await Promise.all(promises.map(p => p.texture));

    for (const [i, texture] of resolved.entries()) {
      const name = promises[i].name;
      this.textures.set(name, texture);
    }
  }

  public async fetchTextures() {
    const promises = [this.loadTexture(tableTexture), this.loadTexture(carpetTexture)];

    const names = ["table", "carpet"];

    const resolved = await Promise.all(promises);

    for (const [i, texture] of resolved.entries()) {
      this.textures.set(names[i], texture);
    }
  }

  public async fetchModels() {
    const promises = [
      this.loadModel(pokerTableSrc),
      this.loadModel(slotMachineSrc),
      this.loadModel(poolTableSrc),
      this.loadModel(pinballSrc),
      this.loadModel(arcadeMachineSrc),
      this.loadModel(airHockeySrc),
      this.loadModel(jukeboxSrc)
    ];

    const names = [
      CustomModel.PokerTable,
      CustomModel.SlotMachine,
      CustomModel.PoolTable,
      CustomModel.Pinball,
      CustomModel.ArcadeMachine,
      CustomModel.AirHockey,
      CustomModel.Jukebox
    ];

    const resolved = await Promise.all(promises);

    for (const [i, gltf] of resolved.entries()) {
      this.models.set(names[i], gltf);
    }
  }

  public getTexture(name: string): THREE.Texture {
    const texture = this.textures.get(name);

    if (!texture) {
      throw new Error(`Texture ${name} not found! :(`);
    }

    return texture;
  }

  public getModel(name: CustomModel): GLTF {
    const model = this.models.get(name);

    if (!model) {
      throw new Error(`Model ${name} not found! :(`);
    }

    return model;
  }
}

export { AssetDatabase };
