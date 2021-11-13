#include <stdio.h>
#include <stdlib.h>
#include <math.h>
#include "wave.h"

int main(void)
{
  MONO_PCM pcm;
  int n, i;
  double f0, gain;
  
  pcm.fs = 44100; /* �W�{�����g�� */
  pcm.bits = 16; /* �ʎq�����x */
  pcm.length = pcm.fs * 1; /* ���f�[�^�̒��� */
  pcm.s = calloc(pcm.length, sizeof(double)); /* ���f�[�^ */
  
  f0 = 500.0; /* ��{���g�� */
  
  /* �m�R�M���g */
  for (i = 1; i <= 44; i++)
  {
    for (n = 0; n < pcm.length; n++)
    {
      pcm.s[n] += 1.0 / i * sin(2.0 * M_PI * i * f0 * n / pcm.fs);
    }
  }
  
  gain = 0.1; /* �Q�C�� */
  
  for (n = 0; n < pcm.length; n++)
  {
    pcm.s[n] *= gain;
  }
  
  wave_write_16bit_mono(&pcm, "ex3_1.wav");
  
  free(pcm.s);
  
  return 0;
}
