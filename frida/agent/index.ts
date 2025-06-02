const RETOUR_ADDRESS = Module.findBaseAddress('eldenring.exe')?.add(0x7c045d);
import fs from "fs";
const writeDataPath = "A:/Code Projects/er-pmod-menus/resources/output.bytes"

function readField(ptr: NativePointer, type: string, offset: number) {
    switch (type) {
        case "float": return ptr.add(offset).readFloat();
        // Can add more types as needed
        default: return "Unknown";
    }
}

function main() {
    if (RETOUR_ADDRESS === undefined) {
        console.debug("Cannot attach");
        return;
    }
    Interceptor.attach(RETOUR_ADDRESS, {
        onEnter: function (args) {
            const dataStart = args[1];
            // Read the 'weight' field at offset 0x10
            try {
                let aob = dataStart.readByteArray(0x103)?.unwrap();
                fs.writeFileSync(writeDataPath, aob, { encoding: null, flag: 'w' })
            } catch (e) {
                console.log("Failed to read weight at", dataStart.add(0x10), e);
            }
        },
        onLeave: function (retval) {
            // No-op
        }
    });
    Interceptor.flush();
}

function setupLogger() {
    let originalLog = console.log;
    let originalDebug = console.debug
    console.log = function () {
        var args = [].slice.call(arguments);
        originalLog.apply(console.log, [getCurrentDateString()].concat(args));
    };
    console.debug = function () {
        var args = [].slice.call(arguments);
        originalDebug.apply(console.debug, [getCurrentDateString()].concat(args));
    };
    function getCurrentDateString() {
        return (new Date()).toISOString() + ' ::';
    }
}

setupLogger();
main();
