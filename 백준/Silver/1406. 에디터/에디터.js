const input = require('fs').readFileSync('/dev/stdin').toString().trim().split('\n');
const leftStack = input[0].split(""); // 커서 왼쪽에 있는 문자열을 관리하는 스택
const rightStack = []; // 커서 오른쪽에 있는 문자열을 관리하는 스택
const editorCommands = input.slice(2);

editorCommands.forEach(command => {
    const [op, char] = command.split(" ");

    if (op === "L" && leftStack.length > 0) {
        rightStack.push(leftStack.pop()); // 왼쪽에서 오른쪽으로 문자 이동
    }
    if (op === "D" && rightStack.length > 0) {
        leftStack.push(rightStack.pop()); // 오른쪽에서 왼쪽으로 문자 이동
    }
    if (op === "B" && leftStack.length > 0) {
        leftStack.pop(); // 왼쪽 스택에서 문자 삭제
    }
    if (op === "P") {
        leftStack.push(char); // 왼쪽 스택에 문자 삽입
    }
});

// 최종 문자열을 만들기 위해 두 스택을 결합
const result = leftStack.concat(rightStack.reverse()).join("");
console.log(result);
