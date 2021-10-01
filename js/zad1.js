const _ = require('lodash');

const tests = [
	[3, [5, 7, 9], [5, 7, 9], 0, true],
	[5, [4,7,1,4,5], [1,4,5,4,7], 2, true],
	[5, [10, 9, 12, 10, 9], [10, 10, 9, 9, 12], 3, false],
	[5, [3, 6, 5, 1, 8], [5, 1, 8, 3, 6], 4, false]
];

const zad1 = (n, A, B, k) => {
	if (k < 0 || n <= k) return false;

	return _.isEqual(A.slice(0, k), B.slice(n-k, n)) && _.isEqual(A.slice(k, n), B.slice(0, n-k));
}

const zad13 = (n, A, B) => {
	for (let i = 0; i < n; i++) {
		if (zad1(n, A, B, i)) {
			return i;
		}
	}

	return -1;
}

tests.forEach((tt, index) => {
	if (zad1(tt[0], tt[1], tt[2], tt[3]) !== tt[4]) {
		console.error(`Test #${index}: zad1(${tt}) !== ${tt[4]}`); 
		process.exit();
	}

	if (tt[4] === true) {
		if (zad13(tt[0], tt[1], tt[2]) !== tt[3]) {
			console.error(`Test #${index}: zad13(${tt}) !== ${tt[3]}`);
			process.exit();
		}
	}

	console.log(`Test #${index}: PASS`);
});


