// SPDX-FileCopyrightText: 2022 Vladimir Rusinov
//
// SPDX-License-Identifier: Apache-2.0

// Ginkgo BDD Testing Framework <http://onsi.github.io/ginkgo></http:>
// Gomega Matcher Library <http://onsi.github.io/gomega></http:>

package kata

import (
	. "github.com/onsi/ginkgo"
	. "github.com/onsi/gomega"
)

var _ = Describe("Sample Test Cases:", func() {
	It("Should return the correct values", func() {
		Expect(toWeirdCase("abc def")).To(Equal("AbC DeF"))
		Expect(toWeirdCase("ABC")).To(Equal("AbC"))
		Expect(toWeirdCase("This is a test Looks like you passed")).To(Equal("ThIs Is A TeSt LoOkS LiKe YoU PaSsEd"))
	})
})
