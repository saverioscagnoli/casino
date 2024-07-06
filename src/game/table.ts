import * as THREE from "three";
import { AssetDatabase } from "~/core/asset-database";
import { Scene } from "~/core/scene";
import { Deck } from "~/game/deck";
import { Player } from "~/game/player";

class Table {
  public static COLOR: number = 0x4e9164;

  private deck: Deck;
  private players: Player[];

  public constructor() {
    this.deck = new Deck();
    this.players = [];
  }

  public init(scene: Scene, playerCount: number) {
    const db = AssetDatabase.build();

    this.deck.init({ scene, shuffled: true });
    this.deck.placeInShuffler();

    const tableGeometry = new THREE.PlaneGeometry(55, 25);
    const tableMaterial = new THREE.MeshLambertMaterial({
      map: db.getTexture("table"),
      side: THREE.DoubleSide
    });

    const tableMesh = new THREE.Mesh(tableGeometry, tableMaterial);

    tableMesh.rotation.x = -Math.PI / 2;

    tableMesh.receiveShadow = true;

    scene.add(tableMesh);

    const positions = [0, 7, -7, 14, -14];

    for (let i = 0; i < playerCount; i++) {
      const player = new Player();
      player.init(scene);
      player.setCardAreaPosition(positions[i] || 0, 0.05, 6 - i / 2);
      this.players.push(player);
    }

    const dealer = new Player({ dealer: true });
    dealer.init(scene);

    dealer.setCardAreaPosition(0, 0.05, -8);
    this.players.push(dealer);

    // Deal a classic blackjack hand
    this.deck
      .dealToAll(this.players, false)
      .then(() => this.deck.dealTo(dealer, true))
      .then(() => this.deck.dealToAll(this.players, false))
      .then(() => this.deck.dealTo(dealer, false));
  }

  public update() {
    this.deck.update();
  }
}

export { Table };
