import (
	"strconv"
	"strings"
)

func solution(s string) int {
	a := []string{"zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"}
	for i, v := range a {
		s = strings.Replace(s, v, strconv.Itoa(i), -1)
	}
    x, _ := strconv.Atoi(s)
	return x
}
