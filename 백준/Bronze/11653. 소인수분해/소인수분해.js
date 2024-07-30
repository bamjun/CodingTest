const input = Number(require('fs').readFileSync('/dev/stdin').toString().trim());

const getPrimeFactors = (n) => {
    return Array.from({length:n},(_,i) => i + 2).reduce((acc,cur) => {
        while(n % cur === 0) {
            acc.push(cur);
            n = n/cur;
        }
        return acc;
    },[])
}

getPrimeFactors(input).forEach((ele)=> console.log(ele))