#ifndef DEBUG_H
#define DEBUG_H

void PrintPhonemes(int call_site, unsigned char *phonemeindex, unsigned char *phonemeLength, unsigned char *stress);
void PrintOutput(int call_site,
                 unsigned char *flag,
                 unsigned char *f1,
                 unsigned char *f2,
                 unsigned char *f3,
                 unsigned char *a1,
                 unsigned char *a2,
                 unsigned char *a3,
                 unsigned char *p);

void PrintRule(int offset);

typedef void (*phonemesTestCallback)(int call_site,
                                     unsigned char *phonemeindex,
                                     unsigned char *phonemeLength,
                                     unsigned char *stress);

void SetPhonemesTestCallback(phonemesTestCallback);

typedef void (*framesTestCallback)(int call_site,
                                   unsigned char *flag,
                                   unsigned char *f1,
                                   unsigned char *f2,
                                   unsigned char *f3,
                                   unsigned char *a1,
                                   unsigned char *a2,
                                   unsigned char *a3,
                                   unsigned char *p);

void SetFramesTestCallback(framesTestCallback);

#endif
