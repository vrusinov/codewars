package kata_test

import (
	"testing"

	. "github.com/onsi/ginkgo"
	. "github.com/onsi/gomega"
)

func TestBouncingBalls(t *testing.T) {
	RegisterFailHandler(Fail)
	RunSpecs(t, "BouncingBalls Suite")
}
