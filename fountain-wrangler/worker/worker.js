addEventListener('fetch', event => {
  event.respondWith(handleRequest(event.request))
})

/**
 * Fetch and log a request
 * @param {Request} request
 */
async function handleRequest(request) {
  // Extract screenplay from POST request body
  if (request.method !== 'POST') {
    return new Response(
      `HTTP verb ${request.method} isn't supported`,
      { status: 400 }
    )
  }
  let j = await request.json();
  const screenplay = j.screenplay
  if (screenplay === null || screenplay === undefined || screenplay == "") {
    return new Response(
      `Body must contain a 'screenplay' field and it cannot be ${screenplay}`,
      { status: 400 }
    )
  }

  // Respond
  const { parse } = wasm_bindgen;
  await wasm_bindgen(wasm)
  const output = parse(screenplay)
  let res = new Response(output, { status: 200 })
  res.headers.set("Content-type", "text/html")
  return res
}