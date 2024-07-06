import * as THREE from "three";
import { mergeGeometries } from "three/examples/jsm/utils/BufferGeometryUtils.js";
import { AssetDatabase } from "~/core/asset-database";
import { EventEmitter } from "~/core/event-emitter";
import { Scene } from "~/core/scene";
import { CardLabel, CardSuit } from "~/lib/enums";
import { getCardName } from "~/lib/utils";

class Card extends EventEmitter {
  public static SPEED: number = 0.1;

  private suit: CardSuit;
  private label: CardLabel;
  private value: number;
  private targetPosition: THREE.Vector3 | null;
  private targetRotation: THREE.Quaternion | null;

  // Whether the card should follow the camera;
  private floating: boolean;

  private static geometry: THREE.PlaneGeometry = new THREE.PlaneGeometry(2, 3.5);

  public mesh!: THREE.Mesh;

  public constructor(suit: CardSuit, label: CardLabel, value: number) {
    super();

    this.suit = suit;
    this.label = label;
    this.value = value;
    this.targetPosition = null;
    this.targetRotation = null;
    this.floating = false;

    this.createMesh(AssetDatabase.build());
  }

  private createMesh(db: AssetDatabase) {
    const cardName = getCardName(this.suit, this.label);

    const frontTexture = db.getTexture(cardName);
    const backTexture = db.getTexture("back-red");

    const cardFrontMaterial = new THREE.MeshBasicMaterial({
      map: frontTexture,
      transparent: true
    });

    const cardBackMaterial = new THREE.MeshBasicMaterial({
      map: backTexture,
      transparent: true
    });

    const cardMesh = new THREE.Mesh(
      mergeGeometries([Card.geometry, Card.geometry.clone().rotateY(Math.PI)], true),
      [cardFrontMaterial, cardBackMaterial]
    );

    cardMesh.castShadow = true;

    this.mesh = cardMesh;
  }

  public setTargetPosition(pos: THREE.Vector3) {
    this.targetPosition = pos;
  }

  public setTargetRotation(pos: THREE.Vector3) {
    this.targetRotation = new THREE.Quaternion().setFromAxisAngle(pos, -Math.PI / 2);
  }

  public isFloating() {
    return this.floating;
  }

  public setFloating(floating: boolean) {
    this.floating = floating;
  }

  public isMoving() {
    return this.targetPosition !== null;
  }

  public isRotating() {
    return this.targetRotation !== null;
  }

  public move(speed: number) {
    if (this.targetPosition === null) {
      return;
    }

    if (this.mesh.position.distanceTo(this.targetPosition) < 0.01) {
      this.targetPosition = null;
      this.emit("arrived");
      return;
    }

    this.emit("move");
    this.mesh.position.lerp(this.targetPosition, speed);
  }

  public rotate(speed: number) {
    if (this.targetRotation === null) {
      return;
    }

    if (this.mesh.quaternion.angleTo(this.targetRotation) < 0.01) {
      this.targetRotation = null;
      return;
    }

    this.mesh.quaternion.slerp(this.targetRotation, speed);
  }

  public show() {
    this.setTargetRotation(new THREE.Vector3(1, 0, 0));
  }

  public hide() {
    this.setTargetRotation(new THREE.Vector3(-1, 0, 0));
  }

  public update() {
    if (this.isMoving()) {
      this.move(Card.SPEED);
    }

    if (this.isRotating()) {
      this.rotate(Card.SPEED);
    }

    if (this.isFloating()) {
      this.mesh.lookAt(Scene.build().getCamera().position);
      this.mesh.rotation.x = -Math.PI / 2.5;
    }
  }
}

export { Card };
