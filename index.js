async function main() {
    const js = await import("./pkg");

    js.greet("Team!");

    js.draw();

    js.call_me_maybe(message => { console.info(`OMG, they called me back with ${message}`)});
    js.listen_for_keys();
}

main();