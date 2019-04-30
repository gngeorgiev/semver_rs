const semver = require('semver');
const fs = require('fs');
const microtime = require('microtime');
const path = require('path');

const ranges = fs.readFileSync(path.join(__dirname, '../ranges.txt'))
    .toString()
    .split('\n')
    .map(l => l.trim())
    .filter(l => !!l);

new semver.Range('>=1.0.0').test('3.0.0'); //warmup

ranges.forEach(range => {
    const start = microtime.now();
    let ex = false;
    let satisfies = false;
    try {
        const r = new semver.Range(range);
        satisfies = r.test('3.0.0');
    } catch (e) {
        ex = true
    }

    const end = microtime.now();
    console.log(`${range},${satisfies},${ex},${end-start}`);
});
