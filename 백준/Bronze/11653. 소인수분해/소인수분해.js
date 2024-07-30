const fs = require('fs');
const input = Number(fs.readFileSync('/dev/stdin').toString().trim());

const getPrimeFactors = (n) => {
    const factors = [];
    let divisor = 2;
    
    while (n >= divisor * divisor) {
        while (n % divisor === 0) {
            factors.push(divisor);
            n = n / divisor;
        }
        divisor++;
    }

    if (n > 1) {
        factors.push(n);
    }

    return factors;
}

if (input > 1) {
    getPrimeFactors(input).forEach((ele) => console.log(ele));
}
