addEventListener('fetch', event => {
	event.respondWith(handleRequest(event.request))
});

/**
 * Fetch and log a request
 * @param {Request} request
 */
async function handleRequest(request) {
	const url = new URL(request.url);
	console.log(url);

    const { ogp_image } = wasm_bindgen;
    await wasm_bindgen(wasm);

	return new Response(ogp_image(), {status: 200, "content-type": "image/png"});
}
