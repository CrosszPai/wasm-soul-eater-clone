import { Loader } from "@pixi/loaders";

export const loader = new Loader();

// assets will be initialize here
loader.add("forest0", "/assets/bg/forest0.png");
loader.add("forest00", "/assets/bg/forest00.png");
loader.add("bullet", "/assets/enemy/bullet.png");
loader.load(() => {});
export function getResource(name) {
  return loader.resources[name].data;
}
