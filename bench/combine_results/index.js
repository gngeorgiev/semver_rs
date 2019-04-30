const fs = require('fs');
const path = require('path');

const results = [
    {
        name: 'semver_node',
        filename: '../semver_node_res.txt'
    },
    {
        name: 'semver_rs',
        filename: '../semver_rs_res.txt',
    },
    {
        name: 'steveklabnik/semver',
        filename: '../semver_res.txt'
    }
];

class Result {
    constructor(name) {
        this.name = name;
        this.satisfies = 0;
        this.not_satisfies = 0;
        this.errors = 0;
        this.times = [];
        this.average_us = 0;
    }

    calc_average_us() {
        return this.times.reduce((acc, t) => acc + t, 0) / this.times.length;
    }
}

const output = results.map(r => {
    const lines = fs.readFileSync(path.join(__dirname, r.filename)).toString().split('\n').filter(l => !!l);
    const res = lines.reduce((result, line) => {
        const data = line.split(',');

        const time = JSON.parse(data[3]);
        result.times.push(time);

        const error = JSON.parse(data[2]);
        if (error) {
            result.errors++;
            return result;
        }

        const satisfies = JSON.parse(data[1]);
        if (satisfies) {
            result.satisfies++;
        } else {
            result.not_satisfies++;
        }

        return result;
    }, new Result(r.name));

    res.average_us = res.calc_average_us();
    return res;
});

console.table(output, ['name', 'satisfies', 'not_satisfies', 'errors', 'average_us']);