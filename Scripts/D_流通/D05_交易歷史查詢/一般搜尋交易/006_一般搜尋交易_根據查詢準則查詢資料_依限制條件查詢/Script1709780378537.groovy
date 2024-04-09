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

WebUI.setText(findTestObject('Object Repository/流通/交易歷史查詢/一般/依限制條件查詢/Page_(15trunk)/input_(15trunk)_member_id'), 'catc')

WebUI.setEncryptedText(findTestObject('Object Repository/流通/交易歷史查詢/一般/依限制條件查詢/Page_(15trunk)/input_(15trunk)_member_pwd'), 
    'eh9+mX6n5G+m9qEgDyu51A==')

WebUI.enhancedClick(findTestObject('Object Repository/流通/交易歷史查詢/一般/依限制條件查詢/Page_(15trunk)/input_(15trunk)_Submit'))

WebUI.verifyElementText(findTestObject('Object Repository/流通/交易歷史查詢/一般/依限制條件查詢/Page_(15trunk)/a_'), '流通')

WebUI.enhancedClick(findTestObject('Object Repository/流通/交易歷史查詢/一般/依限制條件查詢/Page_(15trunk)/a_'))

WebUI.verifyElementText(findTestObject('Object Repository/流通/交易歷史查詢/一般/依限制條件查詢/Page_(15trunk)/a__1'), '交易歷史查詢')

WebUI.enhancedClick(findTestObject('Object Repository/流通/交易歷史查詢/一般/依限制條件查詢/Page_(15trunk)/a__1'))

WebUI.enhancedClick(findTestObject('Object Repository/流通/交易歷史查詢/一般/依限制條件查詢/Page_(15trunk)/a__1_2'))

WebUI.verifyElementText(findTestObject('Object Repository/流通/交易歷史查詢/一般/依限制條件查詢/Page_(15trunk)/a__1_2_3'), '限制條件')

WebUI.enhancedClick(findTestObject('Object Repository/流通/交易歷史查詢/一般/依限制條件查詢/Page_(15trunk)/a__1_2_3'))

WebUI.verifyElementText(findTestObject('Object Repository/流通/交易歷史查詢/一般/依限制條件查詢/Page_(15trunk)/span_'), '限制條件')

WebUI.enhancedClick(findTestObject('Object Repository/流通/交易歷史查詢/一般/依限制條件查詢/Page_(15trunk)/td_'))

WebUI.verifyElementText(findTestObject('Object Repository/流通/交易歷史查詢/一般/依限制條件查詢/Page_(15trunk)/td_'), '交易狀態類型:')

WebUI.verifyElementText(findTestObject('Object Repository/流通/交易歷史查詢/一般/依限制條件查詢/Page_(15trunk)/td__1'), '交易紀錄類型:')

WebUI.selectOptionByValue(findTestObject('Object Repository/流通/交易歷史查詢/一般/依限制條件查詢/Page_(15trunk)/select_'), '1', true)

WebUI.verifyElementText(findTestObject('Object Repository/流通/交易歷史查詢/一般/依限制條件查詢/Page_(15trunk)/td__1_2'), '交易結束模式:')

WebUI.selectOptionByValue(findTestObject('Object Repository/流通/交易歷史查詢/一般/依限制條件查詢/Page_(15trunk)/select__1'), '1', true)

WebUI.verifyElementText(findTestObject('Object Repository/流通/交易歷史查詢/一般/依限制條件查詢/Page_(15trunk)/td__1_2_3'), '館藏狀態:')

WebUI.selectOptionByValue(findTestObject('Object Repository/流通/交易歷史查詢/一般/依限制條件查詢/Page_(15trunk)/select_PickListValues.15004108CWEN'), 
    '2', true)

WebUI.verifyElementText(findTestObject('Object Repository/流通/交易歷史查詢/一般/依限制條件查詢/Page_(15trunk)/td__1_2_3_4'), '館藏地:')

WebUI.enhancedClick(findTestObject('Object Repository/流通/交易歷史查詢/一般/依限制條件查詢/Page_(15trunk)/img__open'))

WebUI.enhancedClick(findTestObject('Object Repository/流通/交易歷史查詢/一般/依限制條件查詢/Page_(15trunk)/label'))

WebUI.enhancedClick(findTestObject('Object Repository/流通/交易歷史查詢/一般/依限制條件查詢/Page_(15trunk)/button_'))

WebUI.verifyElementText(findTestObject('Object Repository/流通/交易歷史查詢/一般/依限制條件查詢/Page_(15trunk)/td__1_2_3_4_5'), '資料類型:')

WebUI.enhancedClick(findTestObject('Object Repository/流通/交易歷史查詢/一般/依限制條件查詢/Page_(15trunk)/img__open'))

WebUI.enhancedClick(findTestObject('Object Repository/流通/交易歷史查詢/一般/依限制條件查詢/Page_(15trunk)/label_1'))

WebUI.enhancedClick(findTestObject('Object Repository/流通/交易歷史查詢/一般/依限制條件查詢/Page_(15trunk)/label_1_2'))

WebUI.enhancedClick(findTestObject('Object Repository/流通/交易歷史查詢/一般/依限制條件查詢/Page_(15trunk)/label_1_2_3'))

WebUI.enhancedClick(findTestObject('Object Repository/流通/交易歷史查詢/一般/依限制條件查詢/Page_(15trunk)/label_1_2_3_4'))

WebUI.enhancedClick(findTestObject('Object Repository/流通/交易歷史查詢/一般/依限制條件查詢/Page_(15trunk)/label_1_2_3_4_5'))

WebUI.enhancedClick(findTestObject('Object Repository/流通/交易歷史查詢/一般/依限制條件查詢/Page_(15trunk)/label_1_2_3_4_5_6'))

WebUI.enhancedClick(findTestObject('Object Repository/流通/交易歷史查詢/一般/依限制條件查詢/Page_(15trunk)/label_1_2_3_4_5_6_7'))

WebUI.enhancedClick(findTestObject('Object Repository/流通/交易歷史查詢/一般/依限制條件查詢/Page_(15trunk)/label_1_2_3_4_5_6_7_8'))

WebUI.enhancedClick(findTestObject('Object Repository/流通/交易歷史查詢/一般/依限制條件查詢/Page_(15trunk)/label_1_2_3_4_5_6_7_8_9'))

WebUI.enhancedClick(findTestObject('Object Repository/流通/交易歷史查詢/一般/依限制條件查詢/Page_(15trunk)/label_1_2_3_4_5_6_7_8_9_10'))

WebUI.enhancedClick(findTestObject('Object Repository/流通/交易歷史查詢/一般/依限制條件查詢/Page_(15trunk)/button_'))

WebUI.enhancedClick(findTestObject('Object Repository/流通/交易歷史查詢/一般/依限制條件查詢/Page_(15trunk)/img__Any_0_1'))

WebUI.enhancedClick(findTestObject('Object Repository/流通/交易歷史查詢/一般/依限制條件查詢/Page_(15trunk)/a__1_2_3_4'))

WebUI.verifyElementText(findTestObject('Object Repository/流通/交易歷史查詢/一般/依限制條件查詢/Page_(15trunk)/td__1_2_3_4_5_6'), '讀者身份類別:')

WebUI.verifyElementText(findTestObject('Object Repository/流通/交易歷史查詢/一般/依限制條件查詢/Page_(15trunk)/td__1_2_3_4_5_6_7'), '館藏使用類型:')

WebUI.selectOptionByValue(findTestObject('Object Repository/流通/交易歷史查詢/一般/依限制條件查詢/Page_(15trunk)/select_eeewwwtesterer(Normal)(Exhibition)(A_198b70'), 
    '3', true)

WebUI.enhancedClick(findTestObject('Object Repository/流通/交易歷史查詢/一般/依限制條件查詢/Page_(15trunk)/input__formSubmitSearch'))

WebUI.takeFullPageScreenshot()

WebUI.enhancedClick(findTestObject('Object Repository/流通/交易歷史查詢/一般/依限制條件查詢/Page_(15trunk)/a_2'))

WebUI.enhancedClick(findTestObject('Object Repository/流通/交易歷史查詢/一般/依限制條件查詢/Page_(15trunk)/a_3'))

WebUI.enhancedClick(findTestObject('Object Repository/流通/交易歷史查詢/一般/依限制條件查詢/Page_(15trunk)/a_4'))

WebUI.enhancedClick(findTestObject('Object Repository/流通/交易歷史查詢/一般/依限制條件查詢/Page_(15trunk)/a__1_2_3_4_5'))

WebUI.closeBrowser()

