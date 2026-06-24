const CACHE_NAME = "RUSTLE_PWA_CACHE_V1";
const ASSETS_TO_CACHE = [];

const preload = async () => {
  console.log("Installing web app");
  return await caches.open(CACHE_NAME)
    .then(async (cache) => {
      console.log("Caching index and important routes");
      try {
        const response = await fetch("/asset-manifest.json");
        const assets = await response.json();
        ASSETS_TO_CACHE.push(...assets);
      } catch (err) {
        console.warn("Failed to fetch asset manifest, falling back to basic caching:", err);
        ASSETS_TO_CACHE.push("/", "/index.html", "/public/favicon.png", "/public/manifest.json");
      }
      console.log("Assets Cached:", ASSETS_TO_CACHE);
      return cache.addAll(ASSETS_TO_CACHE);
  });
}

globalThis.addEventListener("install", (event) => {
  event.waitUntil(preload());
});

globalThis.addEventListener("activate", (event) => {
  event.waitUntil(clients.claim());
});

globalThis.addEventListener("fetch", (event) => {
  event.respondWith(
    caches.match(event.request).then((cachedResponse) => {
      return cachedResponse || fetch(event.request);
    })
  );
});
