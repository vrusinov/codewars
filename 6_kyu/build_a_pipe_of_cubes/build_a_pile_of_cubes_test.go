// SPDX-FileCopyrightText: 2022 Vladimir Rusinov
//
// SPDX-License-Identifier: Apache-2.0

package kata
import (
  . "github.com/onsi/ginkgo"
  . "github.com/onsi/gomega"
)

func dotest(n int, exp int) {
    var ans = FindNb(n)
    Expect(ans).To(Equal(exp))
}

var _ = Describe("Test Example", func() {

    It("should handle basic cases", func() {
        dotest(1071225, 45)
        dotest(4183059834009, 2022)
        dotest(24723578342962, -1)
    })
})

