/* USER CODE BEGIN Header */
/**
  ******************************************************************************
  * @file           : main.c
  * @brief          : Main program body
  ******************************************************************************
  * @attention
  *
  * Copyright (c) 2025 STMicroelectronics.
  * All rights reserved.
  *
  * This software is licensed under terms that can be found in the LICENSE file
  * in the root directory of this software component.
  * If no LICENSE file comes with this software, it is provided AS-IS.
  *
  ******************************************************************************
  */
/* USER CODE END Header */
/* Includes ------------------------------------------------------------------*/
#include "main.h"
#include "usb_device.h"

/* Private includes ----------------------------------------------------------*/
/* USER CODE BEGIN Includes */
#include <stdlib.h>
#include <stdio.h>
#include <string.h>

#include "usbd_cdc_if.h"
#include "shared_logic.h"
#include <stdbool.h>

/* USER CODE END Includes */

/* Private typedef -----------------------------------------------------------*/
/* USER CODE BEGIN PTD */

/* USER CODE END PTD */

/* Private define ------------------------------------------------------------*/
/* USER CODE BEGIN PD */

/* USER CODE END PD */

/* Private macro -------------------------------------------------------------*/
/* USER CODE BEGIN PM */

/* USER CODE END PM */

/* Private variables ---------------------------------------------------------*/
I2C_HandleTypeDef hi2c1;

SPI_HandleTypeDef hspi1;

UART_HandleTypeDef huart2;

/* USER CODE BEGIN PV */

/* USER CODE END PV */

/* Private function prototypes -----------------------------------------------*/
void SystemClock_Config(void);
static void MX_GPIO_Init(void);
static void MX_I2C1_Init(void);
static void MX_SPI1_Init(void);
static void MX_USART2_UART_Init(void);
/* USER CODE BEGIN PFP */

/* USER CODE END PFP */

/* Private user code ---------------------------------------------------------*/
/* USER CODE BEGIN 0 */
uint8_t rx_buffer[256]; // Adjust size as needed
char buffer[128];
char send_AT_command[64];

enum {
	NOISE_DATA,
	MAGNETOMETER_DATA,
} state = NOISE_DATA;

typedef struct __attribute__((packed)) {
	uint16_t header;
	double latitude;
	double longitude;
	double decibels;
} NoiseData;

void sendNoiseData(double latitude, double longitude, double decibels) {
	NoiseData data = {
		.header = 0xABCD,
		.latitude = latitude,
		.longitude = longitude,
		.decibels = decibels,
	};
    snprintf(buffer, sizeof(buffer), "{\"HEADER\":%u, \"D1\":%.3f, \"D2\":%.3f, \"D3\":%.3f}\n\r", data.header, data.latitude, data.longitude, data.decibels);
    snprintf(send_AT_command, sizeof(send_AT_command), "AT+CIPSEND=0,%u\r\n", strlen(buffer) - 2);
    //CDC_Transmit_FS((uint8_t*)&data, sizeof(NoiseData));
}

double testLatitude = -80;
double testLongitude = -150;
double testDecibels = 50;

void sendTestData() {
	sendNoiseData(testLatitude, testLongitude, testDecibels);

	testLatitude += 20;
	if (testLatitude > 80) {
		testLatitude = -80;
		testLongitude += 50;
		if (testLongitude > 150) testLongitude = -150;
	}
	testDecibels += 10;
	if (testDecibels > 150) testDecibels = 50;
}

uint8_t i2c1Write(uint8_t device, uint8_t reg, uint8_t data) {
	return HAL_I2C_Mem_Write(&hi2c1, device << 1, reg, I2C_MEMADD_SIZE_8BIT, &data, 1, 10);
}

void i2c1Read(uint8_t device, uint8_t reg, uint8_t *data, uint8_t size) {
	if (size > 1 && device == 0x19) reg |= 0x80;
	HAL_I2C_Mem_Read(&hi2c1, device << 1, reg, I2C_MEMADD_SIZE_8BIT, data, size, size);
}

void initMagnetometer() {
	__HAL_I2C_ENABLE(&hi2c1);
	i2c1Write(0x1E, 0x00, 0x10); // 7.5 Hz
	i2c1Write(0x1E, 0x02, 0x00); // Continuous conversion mode
}

typedef struct __attribute__((packed)) {
	uint16_t header;
	uint8_t data[6];
} MagnetometerData;

void HAL_GPIO_EXTI_Callback(uint16_t GPIO_Pin) {
	if (state == MAGNETOMETER_DATA) {
		MagnetometerData data = { .header = 0xBBCD };
		i2c1Read(0x1E, 0x03, data.data, 6);

        uint16_t combined1 = (data.data[0] << 8) | data.data[1];
        uint16_t combined2 = (data.data[2] << 8) | data.data[3];
        uint16_t combined3 = (data.data[4] << 8) | data.data[5];

        snprintf(buffer, sizeof(buffer), "{\"HEADER\":%u, \"D1\":%.3f, \"D2\":%.3f, \"D3\":%.3f}\n\r", data.header, (float)combined1, (float)combined2, (float)combined3);
        snprintf(send_AT_command, sizeof(send_AT_command), "AT+CIPSEND=0,%u\r\n", strlen(buffer) - 2);
        //CDC_Transmit_FS((uint8_t *)&data, sizeof(MagnetometerData));
	}
}

/* USER CODE END 0 */

/**
  * @brief  The application entry point.
  * @retval int
  */
int main(void)
{

  /* USER CODE BEGIN 1 */

  /* USER CODE END 1 */

  /* MCU Configuration--------------------------------------------------------*/

  /* Reset of all peripherals, Initializes the Flash interface and the Systick. */
  HAL_Init();

  /* USER CODE BEGIN Init */

  /* USER CODE END Init */

  /* Configure the system clock */
  SystemClock_Config();

  /* USER CODE BEGIN SysInit */

  /* USER CODE END SysInit */

  /* Initialize all configured peripherals */
  MX_GPIO_Init();
  MX_I2C1_Init();
  MX_SPI1_Init();
  MX_USB_DEVICE_Init();
  MX_USART2_UART_Init();
  /* USER CODE BEGIN 2 */

  initMagnetometer();

  /* USER CODE END 2 */

  /* Infinite loop */
  /* USER CODE BEGIN WHILE */

  GPIO_PinState lastState = GPIO_PIN_RESET;
  uint8_t i = 0;

  int8_t command_counter = 0;
  uint8_t waiting_on_response = 0;
  uint8_t sending_data = 0;
  uint8_t send_command_sent = 0;

  while (1)
  {
	/* UART INTERRUPT CODE */
	  if (received_buffer[0] != 0 && sending_data == 1) sending_data = 0; // Nov ukaz bo ustavil pošiljanje podatkov
	  switch (received_buffer[0]) {
	  	  case 'A': // Inicializiraj
	  		  if (waiting_on_response == 0) {
				  if (command_counter == 0) command_counter = 3;

				  switch (command_counter) {
					  case 3:
						  HAL_UART_Transmit(&huart2, (uint8_t*)"AT+CWMODE=3,0\r\n", strlen("AT+CWMODE=3,0\r\n"), HAL_MAX_DELAY);
						  HAL_UART_Receive_IT(&huart2, (uint8_t*)rx_buffer, sizeof(rx_buffer));
						  waiting_on_response = 1;
						  break;
					  case 2:
						  HAL_UART_Transmit(&huart2, (uint8_t*)"AT+CIPMUX=1\r\n", strlen("AT+CIPMUX=1\r\n"), HAL_MAX_DELAY);
						  HAL_UART_Receive_IT(&huart2, (uint8_t*)rx_buffer, sizeof(rx_buffer));
						  waiting_on_response = 1;
						  break;
					  case 1:
						  HAL_UART_Transmit(&huart2, (uint8_t*)"AT+CWJAP=\"1FDCB3\",\"qq3dmpu8g6\"\r\n", strlen("AT+CWJAP=\"1FDCB3\",\"qq3dmpu8g6\"\r\n"), HAL_MAX_DELAY);
						  HAL_UART_Receive_IT(&huart2, (uint8_t*)rx_buffer, sizeof(rx_buffer));
						  waiting_on_response = 1;
						  break;
					  default:
						  memset(received_buffer, 0, sizeof(received_buffer));
						  break;
				  }
	  		  }


			  HAL_Delay(100);
	          break;
	      case 'B':
	          // Handle case for 'B'
	          break;
	      case 'C':
	          // Handle case for 'C'
	          break;
	      case 'D': // Test ESP
	    	  memset(received_buffer, 0, sizeof(received_buffer));

			  HAL_UART_Transmit(&huart2, (uint8_t*)"AT\r\n", strlen("AT\r\n"), HAL_MAX_DELAY);
			  HAL_UART_Receive_IT(&huart2, (uint8_t*)rx_buffer, sizeof(rx_buffer));

			  HAL_Delay(100);
	          break;
	      case 'E':
	          // Handle case for 'E'
	          break;
	      case 'F':
	          // Handle case for 'F'
	          break;
	      case 'G':
	          // Handle case for 'G'
	          break;
	      case 'H':
	          // Handle case for 'H'
	          break;
	      case 'I':
	          // Handle case for 'I'
	          break;
	      case 'J':
	          // Handle case for 'J'
	          break;
	      case 'K':
	          // Handle case for 'K'
	          break;
	      case 'L':
	          // Handle case for 'L'
	          break;
	      case 'M':
	          // Handle case for 'M'
	          break;
	      case 'N':
	          // Handle case for 'N'
	          break;
	      case 'O':
	          // Handle case for 'O'
	          break;
	      case 'P':
	          // Handle case for 'P'
	          break;
	      case 'Q':
	          // Handle case for 'Q'
	          break;
	      case 'R':
	          // Handle case for 'R'
	          break;
	      case 'S':
	          // Handle case for 'S'
	          break;
	      case 'T':
	          // Handle case for 'T'
	          break;
	      case 'U':
	          // Handle case for 'U'
	          break;
	      case 'V':
	          // Handle case for 'V'
	          break;
	      case 'W':
	          // Handle case for 'W'
	          break;
	      case 'X':
	          // Handle case for 'X'
	          break;
	      case 'Y':
	    	  memset(received_buffer, 0, sizeof(received_buffer));

			  HAL_UART_Transmit(&huart2, (uint8_t*)"AT+CIPSTA?\r\n", strlen("AT+CIPSTA?\r\n"), HAL_MAX_DELAY);
			  HAL_UART_Receive_IT(&huart2, (uint8_t*)rx_buffer, sizeof(rx_buffer));

			  HAL_Delay(100);
	          break;
	      case 'Z': // Posiljanje podatkov
	  		  if (waiting_on_response == 0) {
				  if (command_counter == 0) command_counter = 2;

				  switch (command_counter) {
					  case 2:
						  HAL_UART_Transmit(&huart2, (uint8_t*)"AT+CIPSTART=0,\"TCP\",\"192.168.0.30\",8080\r\n", strlen("AT+CIPSTART=0,\"TCP\",\"192.168.0.30\",8080\r\n"), HAL_MAX_DELAY);
						  HAL_UART_Receive_IT(&huart2, (uint8_t*)rx_buffer, sizeof(rx_buffer));
						  waiting_on_response = 1;
						  break;
					  case 1:
						  sending_data = 1;
						  send_command_sent = 0;
						  memset(received_buffer, 0, sizeof(received_buffer));
						  command_counter = 0;
						  break;
					  default:
						  memset(received_buffer, 0, sizeof(received_buffer));
						  break;
				  }
	  		  }


			  HAL_Delay(100);
	          break;
	      default:
	    	  HAL_UART_AbortReceive_IT(&huart2);
	          break;
	  }

	/* UART INTERRUPT CODE END*/
	/* MAGNETOMETER CODE */

	GPIO_PinState currentState = HAL_GPIO_ReadPin(GPIOA, GPIO_PIN_0);
	if (!lastState && currentState) state = (state + 1) % 2;
	lastState = currentState;

	switch (state) {
	case NOISE_DATA:
		HAL_GPIO_WritePin(GPIOE, GPIO_PIN_8, GPIO_PIN_SET);
		HAL_GPIO_WritePin(GPIOE, GPIO_PIN_9, GPIO_PIN_RESET);
		break;
	case MAGNETOMETER_DATA:
		HAL_GPIO_WritePin(GPIOE, GPIO_PIN_8, GPIO_PIN_RESET);
		HAL_GPIO_WritePin(GPIOE, GPIO_PIN_9, GPIO_PIN_SET);
		break;
	}

	if (state == NOISE_DATA && i == 0) sendTestData();
	i = (i + 1) % 50;

	/* MAGNETOMETER CODE END */

	if (sending_data == 1 && waiting_on_response == 0 && buffer[0] != 0 && send_AT_command[0] != 0) {
		if (send_command_sent == 0) {
		  HAL_UART_Transmit(&huart2, (uint8_t*)send_AT_command, strlen(send_AT_command), HAL_MAX_DELAY);
		  HAL_UART_Receive_IT(&huart2, (uint8_t*)rx_buffer, sizeof(rx_buffer));
		  waiting_on_response = 1;
		  send_command_sent = 1;
		}
		else {
		  HAL_UART_Transmit(&huart2, (uint8_t*)buffer, strlen(buffer), HAL_MAX_DELAY);
		  HAL_UART_Receive_IT(&huart2, (uint8_t*)rx_buffer, sizeof(rx_buffer));
		  waiting_on_response = 1;
		  send_command_sent = 0;

		  memset(buffer, 0, sizeof(buffer));
		  memset(send_AT_command, 0, sizeof(send_AT_command));

		  HAL_Delay(500);
		}
	}

	if (rx_buffer[0] != 0) {
	  HAL_GPIO_WritePin(GPIOE, GPIO_PIN_12, GPIO_PIN_SET);
	  HAL_Delay(1000);
	  HAL_GPIO_WritePin(GPIOE, GPIO_PIN_12, GPIO_PIN_RESET);

	  CDC_Transmit_FS((uint8_t*)rx_buffer, strlen(rx_buffer));
	  memset(rx_buffer, 0, sizeof(rx_buffer));
	  HAL_UART_AbortReceive_IT(&huart2);

	  if (command_counter > 0) command_counter--;
	  if (command_counter <= 0) memset(received_buffer, 0, sizeof(received_buffer));
	  waiting_on_response = 0;
	}

	HAL_Delay(10);

    /* USER CODE END WHILE */

    /* USER CODE BEGIN 3 */
  }
  /* USER CODE END 3 */
}

/**
  * @brief System Clock Configuration
  * @retval None
  */
void SystemClock_Config(void)
{
  RCC_OscInitTypeDef RCC_OscInitStruct = {0};
  RCC_ClkInitTypeDef RCC_ClkInitStruct = {0};
  RCC_PeriphCLKInitTypeDef PeriphClkInit = {0};

  /** Initializes the RCC Oscillators according to the specified parameters
  * in the RCC_OscInitTypeDef structure.
  */
  RCC_OscInitStruct.OscillatorType = RCC_OSCILLATORTYPE_HSI|RCC_OSCILLATORTYPE_HSE;
  RCC_OscInitStruct.HSEState = RCC_HSE_BYPASS;
  RCC_OscInitStruct.HSEPredivValue = RCC_HSE_PREDIV_DIV1;
  RCC_OscInitStruct.HSIState = RCC_HSI_ON;
  RCC_OscInitStruct.HSICalibrationValue = RCC_HSICALIBRATION_DEFAULT;
  RCC_OscInitStruct.PLL.PLLState = RCC_PLL_ON;
  RCC_OscInitStruct.PLL.PLLSource = RCC_PLLSOURCE_HSE;
  RCC_OscInitStruct.PLL.PLLMUL = RCC_PLL_MUL6;
  if (HAL_RCC_OscConfig(&RCC_OscInitStruct) != HAL_OK)
  {
    Error_Handler();
  }

  /** Initializes the CPU, AHB and APB buses clocks
  */
  RCC_ClkInitStruct.ClockType = RCC_CLOCKTYPE_HCLK|RCC_CLOCKTYPE_SYSCLK
                              |RCC_CLOCKTYPE_PCLK1|RCC_CLOCKTYPE_PCLK2;
  RCC_ClkInitStruct.SYSCLKSource = RCC_SYSCLKSOURCE_PLLCLK;
  RCC_ClkInitStruct.AHBCLKDivider = RCC_SYSCLK_DIV1;
  RCC_ClkInitStruct.APB1CLKDivider = RCC_HCLK_DIV2;
  RCC_ClkInitStruct.APB2CLKDivider = RCC_HCLK_DIV1;

  if (HAL_RCC_ClockConfig(&RCC_ClkInitStruct, FLASH_LATENCY_1) != HAL_OK)
  {
    Error_Handler();
  }
  PeriphClkInit.PeriphClockSelection = RCC_PERIPHCLK_USB|RCC_PERIPHCLK_USART2
                              |RCC_PERIPHCLK_I2C1;
  PeriphClkInit.Usart2ClockSelection = RCC_USART2CLKSOURCE_PCLK1;
  PeriphClkInit.I2c1ClockSelection = RCC_I2C1CLKSOURCE_HSI;
  PeriphClkInit.USBClockSelection = RCC_USBCLKSOURCE_PLL;
  if (HAL_RCCEx_PeriphCLKConfig(&PeriphClkInit) != HAL_OK)
  {
    Error_Handler();
  }
}

/**
  * @brief I2C1 Initialization Function
  * @param None
  * @retval None
  */
static void MX_I2C1_Init(void)
{

  /* USER CODE BEGIN I2C1_Init 0 */

  /* USER CODE END I2C1_Init 0 */

  /* USER CODE BEGIN I2C1_Init 1 */

  /* USER CODE END I2C1_Init 1 */
  hi2c1.Instance = I2C1;
  hi2c1.Init.Timing = 0x00201D2B;
  hi2c1.Init.OwnAddress1 = 0;
  hi2c1.Init.AddressingMode = I2C_ADDRESSINGMODE_7BIT;
  hi2c1.Init.DualAddressMode = I2C_DUALADDRESS_DISABLE;
  hi2c1.Init.OwnAddress2 = 0;
  hi2c1.Init.OwnAddress2Masks = I2C_OA2_NOMASK;
  hi2c1.Init.GeneralCallMode = I2C_GENERALCALL_DISABLE;
  hi2c1.Init.NoStretchMode = I2C_NOSTRETCH_DISABLE;
  if (HAL_I2C_Init(&hi2c1) != HAL_OK)
  {
    Error_Handler();
  }

  /** Configure Analogue filter
  */
  if (HAL_I2CEx_ConfigAnalogFilter(&hi2c1, I2C_ANALOGFILTER_ENABLE) != HAL_OK)
  {
    Error_Handler();
  }

  /** Configure Digital filter
  */
  if (HAL_I2CEx_ConfigDigitalFilter(&hi2c1, 0) != HAL_OK)
  {
    Error_Handler();
  }
  /* USER CODE BEGIN I2C1_Init 2 */

  /* USER CODE END I2C1_Init 2 */

}

/**
  * @brief SPI1 Initialization Function
  * @param None
  * @retval None
  */
static void MX_SPI1_Init(void)
{

  /* USER CODE BEGIN SPI1_Init 0 */

  /* USER CODE END SPI1_Init 0 */

  /* USER CODE BEGIN SPI1_Init 1 */

  /* USER CODE END SPI1_Init 1 */
  /* SPI1 parameter configuration*/
  hspi1.Instance = SPI1;
  hspi1.Init.Mode = SPI_MODE_MASTER;
  hspi1.Init.Direction = SPI_DIRECTION_2LINES;
  hspi1.Init.DataSize = SPI_DATASIZE_4BIT;
  hspi1.Init.CLKPolarity = SPI_POLARITY_LOW;
  hspi1.Init.CLKPhase = SPI_PHASE_1EDGE;
  hspi1.Init.NSS = SPI_NSS_SOFT;
  hspi1.Init.BaudRatePrescaler = SPI_BAUDRATEPRESCALER_4;
  hspi1.Init.FirstBit = SPI_FIRSTBIT_MSB;
  hspi1.Init.TIMode = SPI_TIMODE_DISABLE;
  hspi1.Init.CRCCalculation = SPI_CRCCALCULATION_DISABLE;
  hspi1.Init.CRCPolynomial = 7;
  hspi1.Init.CRCLength = SPI_CRC_LENGTH_DATASIZE;
  hspi1.Init.NSSPMode = SPI_NSS_PULSE_ENABLE;
  if (HAL_SPI_Init(&hspi1) != HAL_OK)
  {
    Error_Handler();
  }
  /* USER CODE BEGIN SPI1_Init 2 */

  /* USER CODE END SPI1_Init 2 */

}

/**
  * @brief USART2 Initialization Function
  * @param None
  * @retval None
  */
static void MX_USART2_UART_Init(void)
{

  /* USER CODE BEGIN USART2_Init 0 */

  /* USER CODE END USART2_Init 0 */

  /* USER CODE BEGIN USART2_Init 1 */

  /* USER CODE END USART2_Init 1 */
  huart2.Instance = USART2;
  huart2.Init.BaudRate = 115200;
  huart2.Init.WordLength = UART_WORDLENGTH_8B;
  huart2.Init.StopBits = UART_STOPBITS_1;
  huart2.Init.Parity = UART_PARITY_NONE;
  huart2.Init.Mode = UART_MODE_TX_RX;
  huart2.Init.HwFlowCtl = UART_HWCONTROL_NONE;
  huart2.Init.OverSampling = UART_OVERSAMPLING_16;
  huart2.Init.OneBitSampling = UART_ONE_BIT_SAMPLE_DISABLE;
  huart2.AdvancedInit.AdvFeatureInit = UART_ADVFEATURE_NO_INIT;
  if (HAL_UART_Init(&huart2) != HAL_OK)
  {
    Error_Handler();
  }
  /* USER CODE BEGIN USART2_Init 2 */

  /* USER CODE END USART2_Init 2 */

}

/**
  * @brief GPIO Initialization Function
  * @param None
  * @retval None
  */
static void MX_GPIO_Init(void)
{
  GPIO_InitTypeDef GPIO_InitStruct = {0};
/* USER CODE BEGIN MX_GPIO_Init_1 */
/* USER CODE END MX_GPIO_Init_1 */

  /* GPIO Ports Clock Enable */
  __HAL_RCC_GPIOE_CLK_ENABLE();
  __HAL_RCC_GPIOC_CLK_ENABLE();
  __HAL_RCC_GPIOF_CLK_ENABLE();
  __HAL_RCC_GPIOA_CLK_ENABLE();
  __HAL_RCC_GPIOB_CLK_ENABLE();

  /*Configure GPIO pin Output Level */
  HAL_GPIO_WritePin(GPIOE, CS_I2C_SPI_Pin|LD4_Pin|LD3_Pin|LD5_Pin
                          |LD7_Pin|LD9_Pin|LD10_Pin|LD8_Pin
                          |LD6_Pin, GPIO_PIN_RESET);

  /*Configure GPIO pin : DRDY_Pin */
  GPIO_InitStruct.Pin = DRDY_Pin;
  GPIO_InitStruct.Mode = GPIO_MODE_IT_RISING_FALLING;
  GPIO_InitStruct.Pull = GPIO_NOPULL;
  HAL_GPIO_Init(DRDY_GPIO_Port, &GPIO_InitStruct);

  /*Configure GPIO pins : CS_I2C_SPI_Pin LD4_Pin LD3_Pin LD5_Pin
                           LD7_Pin LD9_Pin LD10_Pin LD8_Pin
                           LD6_Pin */
  GPIO_InitStruct.Pin = CS_I2C_SPI_Pin|LD4_Pin|LD3_Pin|LD5_Pin
                          |LD7_Pin|LD9_Pin|LD10_Pin|LD8_Pin
                          |LD6_Pin;
  GPIO_InitStruct.Mode = GPIO_MODE_OUTPUT_PP;
  GPIO_InitStruct.Pull = GPIO_NOPULL;
  GPIO_InitStruct.Speed = GPIO_SPEED_FREQ_LOW;
  HAL_GPIO_Init(GPIOE, &GPIO_InitStruct);

  /*Configure GPIO pins : MEMS_INT3_Pin MEMS_INT4_Pin MEMS_INT1_Pin MEMS_INT2_Pin */
  GPIO_InitStruct.Pin = MEMS_INT3_Pin|MEMS_INT4_Pin|MEMS_INT1_Pin|MEMS_INT2_Pin;
  GPIO_InitStruct.Mode = GPIO_MODE_EVT_RISING;
  GPIO_InitStruct.Pull = GPIO_NOPULL;
  HAL_GPIO_Init(GPIOE, &GPIO_InitStruct);

  /*Configure GPIO pin : B1_Pin */
  GPIO_InitStruct.Pin = B1_Pin;
  GPIO_InitStruct.Mode = GPIO_MODE_INPUT;
  GPIO_InitStruct.Pull = GPIO_NOPULL;
  HAL_GPIO_Init(B1_GPIO_Port, &GPIO_InitStruct);

  /* EXTI interrupt init*/
  HAL_NVIC_SetPriority(EXTI2_TSC_IRQn, 0, 0);
  HAL_NVIC_EnableIRQ(EXTI2_TSC_IRQn);

/* USER CODE BEGIN MX_GPIO_Init_2 */
/* USER CODE END MX_GPIO_Init_2 */
}

/* USER CODE BEGIN 4 */

/* USER CODE END 4 */

/**
  * @brief  This function is executed in case of error occurrence.
  * @retval None
  */
void Error_Handler(void)
{
  /* USER CODE BEGIN Error_Handler_Debug */
  /* User can add his own implementation to report the HAL error return state */
  __disable_irq();
  while (1)
  {
  }
  /* USER CODE END Error_Handler_Debug */
}

#ifdef  USE_FULL_ASSERT
/**
  * @brief  Reports the name of the source file and the source line number
  *         where the assert_param error has occurred.
  * @param  file: pointer to the source file name
  * @param  line: assert_param error line source number
  * @retval None
  */
void assert_failed(uint8_t *file, uint32_t line)
{
  /* USER CODE BEGIN 6 */
  /* User can add his own implementation to report the file name and line number,
     ex: printf("Wrong parameters value: file %s on line %d\r\n", file, line) */
  /* USER CODE END 6 */
}
#endif /* USE_FULL_ASSERT */
