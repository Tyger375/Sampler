def get_closest_multiple(value, close):
    if close > value:
        return value
    if value % close == 0:
        return close
    """
    remain = value % close
    if remain == 0:
        return close
    return 
    """
    return max(round(close / value) * value, 1)

for proj_loops in range(1, 11):
    print("----------")
    for track_loops in range(1, proj_loops + 1):
        value = get_closest_multiple(proj_loops, track_loops)
        print(proj_loops, track_loops, value)
        #if (proj_loops % value) != 0:
            #print("ERROR")