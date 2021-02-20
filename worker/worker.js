const FONT_URL = "https://cdn.plusminus.io/font/webkoruri/20140628/WebKoruri.ttf";

addEventListener('fetch', event => {
	event.respondWith(handleRequest(event.request))
});

/**
 * Fetch and log a request
 * @param {Request} request
 */
async function handleRequest(request) {
	const url = new URL(request.url);
	//console.log(url);

    const { ogp_image } = wasm_bindgen;
    await wasm_bindgen(wasm);

	const fr = new Request(FONT_URL, {headers: request.headers});

	const f = fetch(fr, {cf: {}})
		.then(async res => {
			//console.log(res);
			const buff = await res.arrayBuffer();
			const b = new Uint8Array(buff);

			//console.log(buff);
			//console.log(b);

			const ogp = ogp_image(b);
			//console.log(ogp);
			return new Response(ogp, {status: 200, "content-type": "image/png"});
		});
	return await f;

	//return new Response(ogp_image(f.body), {status: 200, "content-type": "image/png"});
}
