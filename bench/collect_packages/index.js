const glob = require('glob');
const path = require('path');

glob('**/package.json', (err, packages) => {
    const ranges = packages.map(p => require(path.resolve('.', p))).reduce((ranges, pkg) => {
        Object.values(pkg.dependencies || {}).forEach(v => ranges.add(v));
        return ranges;
    }, new Set());

    for (const range of ranges.keys()) {
        console.log(range);
    }
});