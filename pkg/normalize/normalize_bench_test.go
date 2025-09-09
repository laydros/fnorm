package normalize

import "testing"

func BenchmarkNormalize(b *testing.B) {
	for i := 0; i < b.N; i++ {
		Normalize("My Complex File Name (Copy) #1.PDF")
	}
}
