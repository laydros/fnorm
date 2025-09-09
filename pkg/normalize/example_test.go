package normalize_test

import (
	"fmt"

	"github.com/laydros/fnorm/pkg/normalize" //nolint:depguard // allow self import
)

func ExampleNormalize() {
	result := normalize.Normalize("My File.PDF")
	fmt.Println(result)
	// Output: my-file.pdf
}
