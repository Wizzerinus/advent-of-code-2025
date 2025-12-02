BEGIN { sum = 0; sum2 = 0 }

function strrep(str, n) {
    if (n == 1) {
        return str
    } else {
        return str strrep(str, n - 1)
    }
}

function repeated(s) {
    for (i = 2; i <= length(s); i++) {
        if (length(s) % i == 0 && strrep(substr(s, 1, length(s) / i), i) == s) {
            return 1
        }
    }
    return 0
}

function countq(fst, snd, r, checkrep) {
    if (length(fst) % r != 0) {
        return 0
    }

    total = 0

    fstn = strtonum(fst)
    sndn = strtonum(snd)
    if (sndn < fstn) {
        print "Not supported: more<less"
        exit(1)
    }

    beginning = substr(fst, 1, length(fst) / r)

    while (1) {
        num = strtonum(strrep(beginning, r))
        rep = (!checkrep) && repeated(beginning)
        beginning = "" (strtonum(beginning) + 1)
        if (num < fstn) {
            continue
        } else if (num > sndn) {
            break
        } else if (!rep) {
            total += num
        }
    }

    return total
}

function count(fst, snd) {
    if (length(snd) - length(fst) > 1) {
        print "Not supported: len>1"
        exit(1)
    }

    if (length(fst) != length(snd)) {
        fst1 = fst
        gsub(".", "9", fst1)
        count(fst, fst1)
        snd1 = snd
        gsub(".", "0", snd1)
        sub(".", "1", snd1)
        count(snd1, snd)
        return
    }

    sum += countq(fst, snd, 2, 1)
    for (j = 2; j <= length(fst); j++) {
        sum2 += countq(fst, snd, j, 0)
    }
}

{count($1, $2)}

END {
    print "Task 1: ", sum
    print "Task 2: ", sum2
}

