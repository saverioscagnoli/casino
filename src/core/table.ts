import { Deck } from "~/core/deck";
import { Player } from "~/core/player";
import { Scene } from "~/core/scene";

import * as THREE from "three";

class Table {
  private deck!: Deck;
  private players!: Player[];
  private turn!: Player;

  public static async create() {
    const table = new Table();

    table.deck = await Deck.create();
    table.players = [];

    return table;
  }

  public init(scene: Scene, playerCount: number) {
    const tableGeometry = new THREE.PlaneGeometry(55, 20);
    const tableMaterial = new THREE.MeshBasicMaterial({
      color: Scene.TABLE_COLOR,
      side: THREE.DoubleSide
    });
    const tableMesh = new THREE.Mesh(tableGeometry, tableMaterial);

    tableMesh.rotation.x = -Math.PI / 2;
    scene.add(tableMesh);

    this.deck.shuffle();

    for (const [i, card] of this.deck.iter()) {
      card.getMesh().position.set(12 + i * 0.1, 1, -6);

      card.getMesh().rotation.x = -Math.PI / 2;
      card.getMesh().rotation.y = Math.PI / 2;

      scene.add(card.getMesh());
    }

    for (let i = 0; i < playerCount; i++) {
      const player = new Player(false);
      player.init(scene, i, playerCount);

      this.addPlayer(player);
    }

    const dealer = new Player(true);
    dealer.init(scene, -1, -1);

    this.addPlayer(dealer);

    for (const [i, player] of this.players.entries()) {
      setTimeout(() => {
        this.deck.draw(2, player);
      }, 500 * i);
    }
  }

  public addPlayer(player: Player) {
    if (this.players.length === 0) {
      this.turn = player;
    }

    this.players.push(player);
  }

  public update() {
    this.deck.update();
  }
}

export { Table };
