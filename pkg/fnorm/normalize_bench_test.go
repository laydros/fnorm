package fnorm

import "testing"

func BenchmarkNormalizeFilename(b *testing.B) {
	for i := 0; i < b.N; i++ {
		Normalize("My Complex File Name (Copy) #1.PDF")
	}
}
