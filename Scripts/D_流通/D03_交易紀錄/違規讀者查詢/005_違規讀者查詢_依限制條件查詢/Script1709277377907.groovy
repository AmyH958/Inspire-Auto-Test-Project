import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

WebUI.openBrowser('')

WebUI.navigateToUrl('http://10.11.20.15/inspireapp/logon')

WebUI.maximizeWindow()

WebUI.setText(findTestObject('Object Repository/流通/交易紀錄/依限制條件查詢/Page_(15trunk)/input_(15trunk)_member_id'), 'catc')

WebUI.setEncryptedText(findTestObject('Object Repository/流通/交易紀錄/依限制條件查詢/Page_(15trunk)/input_(15trunk)_member_pwd'), 'eh9+mX6n5G+m9qEgDyu51A==')

WebUI.enhancedClick(findTestObject('Object Repository/流通/交易紀錄/依限制條件查詢/Page_(15trunk)/input_(15trunk)_Submit'))

WebUI.verifyElementText(findTestObject('Object Repository/流通/交易紀錄/依限制條件查詢/Page_(15trunk)/a_'), '流通')

WebUI.enhancedClick(findTestObject('Object Repository/流通/交易紀錄/依限制條件查詢/Page_(15trunk)/a_'))

WebUI.verifyElementText(findTestObject('Object Repository/流通/交易紀錄/依限制條件查詢/Page_(15trunk)/a__1'), '交易紀錄')

WebUI.enhancedClick(findTestObject('Object Repository/流通/交易紀錄/依限制條件查詢/Page_(15trunk)/a__1'))

WebUI.verifyElementText(findTestObject('Object Repository/流通/交易紀錄/依限制條件查詢/Page_(15trunk)/a__1_2'), '違規讀者查詢')

WebUI.enhancedClick(findTestObject('Object Repository/流通/交易紀錄/依限制條件查詢/Page_(15trunk)/a__1_2'))

WebUI.verifyElementText(findTestObject('Object Repository/流通/交易紀錄/依限制條件查詢/Page_(15trunk)/b_'), '查詢')

WebUI.verifyElementText(findTestObject('Object Repository/流通/交易紀錄/依限制條件查詢/Page_(15trunk)/a__1_2_3'), '限制條件')

WebUI.enhancedClick(findTestObject('Object Repository/流通/交易紀錄/依限制條件查詢/Page_(15trunk)/a__1_2_3'))

WebUI.verifyElementText(findTestObject('Object Repository/流通/交易紀錄/依限制條件查詢/Page_(15trunk)/td_'), '讀者狀態:')

WebUI.selectOptionByValue(findTestObject('Object Repository/流通/交易紀錄/依限制條件查詢/Page_(15trunk)/select_MATLAB1PHP'), '1', true)

WebUI.verifyElementText(findTestObject('Object Repository/流通/交易紀錄/依限制條件查詢/Page_(15trunk)/td__1'), '預設所屬圖書館:')

WebUI.enhancedClick(findTestObject('Object Repository/流通/交易紀錄/依限制條件查詢/Page_(15trunk)/img__open'))

WebUI.enhancedClick(findTestObject('Object Repository/流通/交易紀錄/依限制條件查詢/Page_(15trunk)/label'))

WebUI.enhancedClick(findTestObject('Object Repository/流通/交易紀錄/依限制條件查詢/Page_(15trunk)/button_'))

WebUI.verifyElementText(findTestObject('Object Repository/流通/交易紀錄/依限制條件查詢/Page_(15trunk)/td__1_2'), '讀者其他館別:')

WebUI.enhancedClick(findTestObject('Object Repository/流通/交易紀錄/依限制條件查詢/Page_(15trunk)/img__open'))

WebUI.enhancedClick(findTestObject('Object Repository/流通/交易紀錄/依限制條件查詢/Page_(15trunk)/label'))

WebUI.enhancedClick(findTestObject('Object Repository/流通/交易紀錄/依限制條件查詢/Page_(15trunk)/button_'))

WebUI.enhancedClick(findTestObject('Object Repository/流通/交易紀錄/依限制條件查詢/Page_(15trunk)/input__elementName'))

WebUI.verifyElementText(findTestObject('Object Repository/流通/交易紀錄/依限制條件查詢/Page_(15trunk)/td__1_2_3'), '所系單位:')

WebUI.enhancedClick(findTestObject('Object Repository/流通/交易紀錄/依限制條件查詢/Page_(15trunk)/img__Any_0_1'))

WebUI.enhancedClick(findTestObject('Object Repository/流通/交易紀錄/依限制條件查詢/Page_(15trunk)/a__1_2_3_4'))

WebUI.verifyElementText(findTestObject('Object Repository/流通/交易紀錄/依限制條件查詢/Page_(15trunk)/td__1_2_3_4'), '讀者身份類別:')

WebUI.selectOptionByValue(findTestObject('Object Repository/流通/交易紀錄/依限制條件查詢/Page_(15trunk)/select_010203040506070809101111112131415161_e7543f'), 
    '1', true)

WebUI.enhancedClick(findTestObject('Object Repository/流通/交易紀錄/依限制條件查詢/Page_(15trunk)/input__formSubmitSearch'))

WebUI.enhancedClick(findTestObject('Object Repository/流通/交易紀錄/依限制條件查詢/Page_(15trunk)/div_1                                      _e22bde'))

WebUI.verifyElementPresent(findTestObject('Object Repository/流通/交易紀錄/依限制條件查詢/Page_(15trunk)/div_1                                      _e22bde'), 
    0)

WebUI.verifyElementText(findTestObject('Object Repository/流通/交易紀錄/依限制條件查詢/Page_(15trunk)/td_01'), '01大學生一年級')

WebUI.takeFullPageScreenshotAsCheckpoint('顯示應對應資料')

WebUI.enhancedClick(findTestObject('Object Repository/流通/交易紀錄/依限制條件查詢/Page_(15trunk)/a__1_2_3_4_5'))

WebUI.closeBrowser()

