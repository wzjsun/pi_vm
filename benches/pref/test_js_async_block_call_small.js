var seqArgs = new Uint8Array([0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff]);

function test() {
    var i, index, r;

    for(i = 0; i < 1e1; i++) {
        r = NativeObject.call(0x1, ["test_async_block_call", seqArgs]);
        r = __thread_yield();
    }
    __gc();
}