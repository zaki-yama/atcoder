N,S=map(int,input().split())
A=list(map(int,input().split()))

idx0=[i for i,a in enumerate(A) if a==0]

if len(idx0)==0:
  print(0)
  exit()

L=min(S,idx0[0])
R=max(S,idx0[-1]+1)
print(sum(A[L:R])+(R-L))
