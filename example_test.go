package fnorm_test

import (
	"fmt"

	"github.com/laydros/fnorm" //nolint:depguard // allow self import
)

func ExampleNormalize() {
	result := fnorm.Normalize("My File.PDF")
	fmt.Println(result)
	// Output: my-file.pdf
}
