#include <stdio.h>
#include <stdlib.h>
#include <math.h>
#include "wave.h"

int main(void)
{
  MONO_PCM pcm;
  int n, i;
  double f0, theta, gain;
  
  pcm.fs = 44100; /* �W�{�����g�� */
  pcm.bits = 16; /* �ʎq�����x */
  pcm.length = pcm.fs * 1; /* ���f�[�^�̒��� */
  pcm.s = calloc(pcm.length, sizeof(double)); /* ���f�[�^ */
  
  f0 = 1.0; /* ��{���g�� */
  
  /* ���F�G�� */
  for (i = 1; i <= 22050; i++)
  {
    theta = (double)rand() / RAND_MAX * 2.0 * M_PI;
    
    for (n = 0; n < pcm.length; n++)
    {
      pcm.s[n] += sin(2.0 * M_PI * i * f0 * n / pcm.fs + theta);
    }
  }
  
  gain = 0.001; /* �Q�C�� */
  
  for (n = 0; n < pcm.length; n++)
  {
    pcm.s[n] *= gain;
  }
  
  wave_write_16bit_mono(&pcm, "ex3_5.wav");
  
  free(pcm.s);
  
  return 0;
}
