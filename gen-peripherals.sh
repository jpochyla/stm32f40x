#!/bin/sh
# Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
# http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
# <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
# option. This file may not be copied, modified, or distributed
# except according to those terms.
#
# Author: Jorge Aparicio <japaricious@gmail.com>
# Author: Brandon Edens <brandonedens@gmail.com>
# Author: Jan Pochyla <jpochyla@gmail.com>
# Author: Pavol Rusnak <stick@gk2.sk>
# Date: 2017-02-12

set -ex

SVD=STM32F40x.svd

if [ ! -f $SVD ]; then
    curl -LOs https://github.com/posborne/cmsis-svd/raw/master/data/STMicro/$SVD
fi

svd2rust -i $SVD > src/lib.rs
rustfmt --write-mode overwrite src/lib.rs || :
xargo build --target thumbv7em-none-eabihf
