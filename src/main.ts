import * as THREE from "three";
import {
  EffectComposer,
  GLTFLoader,
  RenderPixelatedPass,
} from "three/examples/jsm/Addons.js";

// @ts-ignore
import poolTableSrc from "~/assets/pool-table/scene.gltf";
// @ts-ignore
import rouletteTableSrc from "~/assets/roulette-table/scene.gltf";
// @ts-ignore
import cardSrc from "~/assets/cards/1-H.png";

// @ts-ignore
import dealerSrc from "~/assets/dealer/scene.gltf";

import { Card } from "~/core/card";
import { Deck } from "./core/deck";

import cardFlipAudioSrc from "~/assets/sounds/card-flip.mp3";

const cardFlipAudio = new Audio(cardFlipAudioSrc);

const screenResolution = new THREE.Vector2(
  window.innerWidth,
  window.innerHeight
);

const aspectRatio = screenResolution.x / screenResolution.y;

const scene = new THREE.Scene();
const camera = new THREE.PerspectiveCamera(75, aspectRatio, 0.1, 1000);

camera.position.set(0, 10, 10);
camera.lookAt(scene.position);

const renderer = new THREE.WebGLRenderer({ antialias: false });

renderer.setSize(screenResolution.x, screenResolution.y);
renderer.setPixelRatio(window.devicePixelRatio);
renderer.shadowMap.enabled = true;
renderer.shadowMap.type = THREE.PCFSoftShadowMap;

document.body.appendChild(renderer.domElement);

const composer = new EffectComposer(renderer);
const renderPass = new RenderPixelatedPass(2, scene, camera);

composer.addPass(renderPass);

const tableGeometry = new THREE.PlaneGeometry(40, 20);
const tableMaterial = new THREE.MeshBasicMaterial({
  color: 0x4e9164,
  side: THREE.DoubleSide,
});
const tableMesh = new THREE.Mesh(tableGeometry, tableMaterial);

tableMesh.rotation.x = -Math.PI / 2;

const deck = await Deck.create();

deck.shuffle();

// ...

for (const [i, card] of deck.getCards().entries()) {
  card.mesh.position.set(12 + i * 0.1, 1, -6);

  card.mesh.rotation.x = -Math.PI / 2;
  card.mesh.rotation.y = Math.PI / 2;

  scene.add(card.mesh);
}

const loader = new GLTFLoader();

const ambientLight = new THREE.AmbientLight(0xffffff, 2);
scene.add(ambientLight);

loader.load(poolTableSrc, (gltf) => {
  gltf.scene.position.set(10, 0, -20);
  gltf.scene.castShadow = true;
  gltf.scene.receiveShadow = true;

  scene.add(gltf.scene);
});

loader.load(dealerSrc, (gltf) => {
  // Set the dealer's position
  gltf.scene.position.set(0, 3, -5);

  gltf.scene.castShadow = true;
  gltf.scene.receiveShadow = true;

  gltf.scene.scale.set(0.1, 0.1, 0.1);

  scene.add(gltf.scene);
});

scene.add(tableMesh);

const targetRotation = new THREE.Quaternion().setFromAxisAngle(
  new THREE.Vector3(1, 0, 0),
  -Math.PI / 2
);

const speed = 0.1;
const cardSoundPlayed = Array(6).fill(false); // Create an array to track if the sound has been played for each card

function moveCard(index: number) {
  const targetPosition = new THREE.Vector3(0 + index / 2, 1 + index * 0.01, 4);

  deck.getCards()[index].mesh.position.lerp(targetPosition, speed);
  deck.getCards()[index].mesh.quaternion.slerp(targetRotation, speed);

  // Play sound only if it hasn't been played for this card
  if (!cardSoundPlayed[index]) {
    cardFlipAudio.play();
    cardSoundPlayed[index] = true;
  }
}

function animate() {
  requestAnimationFrame(animate);

  for (let i = 1; i <= 5; i++) {
    setTimeout(() => moveCard(i), i * 500); // Delay each card's movement by 1 second
  }

  composer.render();
}

animate();
