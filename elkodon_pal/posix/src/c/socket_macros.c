#ifdef _WIN64
#include <WinSock2.h>
#include <Windows.h>
#include <MSWSock.h>
#include <io.h>



#else
#include <sys/select.h>
#include <sys/socket.h>

size_t elkodon_cmsg_space(const size_t len) { return CMSG_SPACE(len); }

struct cmsghdr* elkodon_cmsg_firsthdr(const struct msghdr* hdr) {
    return CMSG_FIRSTHDR(hdr);
}

struct cmsghdr* elkodon_cmsg_nxthdr(struct msghdr* hdr, struct cmsghdr* sub) {
    return CMSG_NXTHDR(hdr, sub);
}

size_t elkodon_cmsg_len(const size_t len) { return CMSG_LEN(len); }

unsigned char* elkodon_cmsg_data(struct cmsghdr* cmsg) {
    return CMSG_DATA(cmsg);
}

void elkodon_fd_clr(const int fd, fd_set* set) { FD_CLR(fd, set); }

int elkodon_fd_isset(const int fd, const fd_set* set) {
    return FD_ISSET(fd, set);
}

void elkodon_fd_set(const int fd, fd_set* set) { FD_SET(fd, set); }

void elkodon_fd_zero(fd_set* set) { FD_ZERO(set); }

#endif
