#include <stdio.h>
#include <stdlib.h>
#include "wave.h"

int main(void)
{
  MONO_PCM pcm0, pcm1;
  int n;
  
  wave_read_16bit_mono(&pcm0, "a.wav"); /* ���f�[�^�̓��� */
  
  pcm1.fs = pcm0.fs; /* �W�{�����g�� */
  pcm1.bits = pcm0.bits; /* �ʎq�����x */
  pcm1.length = pcm0.length; /* ���f�[�^�̒��� */
  pcm1.s = calloc(pcm1.length, sizeof(double)); /* ���f�[�^ */
  
  for (n = 0; n < pcm1.length; n++)
  {
    pcm1.s[n] = pcm0.s[n]; /* ���f�[�^�̃R�s�[ */
  }
  
  wave_write_16bit_mono(&pcm1, "b.wav"); /* ���f�[�^�̏o�� */
  
  free(pcm0.s);
  free(pcm1.s);
  
  return 0;
}
