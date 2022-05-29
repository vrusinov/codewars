package kata_test

import (
	"testing"

	. "github.com/onsi/ginkgo"
	. "github.com/onsi/gomega"
)

func TestBuildAPipeOfCubes(t *testing.T) {
	RegisterFailHandler(Fail)
	RunSpecs(t, "BuildAPipeOfCubes Suite")
}
