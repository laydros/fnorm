package normalize_test

import (
	"fmt"

	normalize "github.com/laydros/fnorm/internal/normalize" //nolint:depguard // internal package import
)

func ExampleNormalize() {
	result := normalize.Normalize("My File.PDF")
	fmt.Println(result)
	// Output: my-file.pdf
}
