#!/bin/bash
F=${1:?}
DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
TMPFILE=$(mktemp /tmp/license.XXXXXX)
cp ${DIR:?}/LICENSE.header ${TMPFILE:?}
cat ${F:?} >> ${TMPFILE:?}
mv ${TMPFILE:?} ${F:?}
