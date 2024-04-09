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

WebUI.setText(findTestObject('Object Repository/流通/借閱台/清除讀者資料/Page_(15trunk)/input_(15trunk)_member_id'), 'catc')

WebUI.setEncryptedText(findTestObject('Object Repository/流通/借閱台/清除讀者資料/Page_(15trunk)/input_(15trunk)_member_pwd'), 'eh9+mX6n5G+m9qEgDyu51A==')

WebUI.click(findTestObject('Object Repository/流通/借閱台/清除讀者資料/Page_(15trunk)/input_(15trunk)_Submit'))

WebUI.verifyElementText(findTestObject('Object Repository/流通/借閱台/清除讀者資料/Page_(15trunk)/a_'), '流通')

WebUI.click(findTestObject('Object Repository/流通/借閱台/清除讀者資料/Page_(15trunk)/a_'))

WebUI.verifyElementText(findTestObject('Object Repository/流通/借閱台/清除讀者資料/Page_(15trunk)/a__1'), '借閱檯')

WebUI.click(findTestObject('Object Repository/流通/借閱台/清除讀者資料/Page_(15trunk)/a__1'))

WebUI.verifyElementText(findTestObject('Object Repository/流通/借閱台/清除讀者資料/Page_(15trunk)/a__1_2'), '讀者流通檯')

WebUI.click(findTestObject('Object Repository/流通/借閱台/清除讀者資料/Page_(15trunk)/a__1_2'))

WebUI.setText(findTestObject('Object Repository/流通/借閱台/清除讀者資料/Page_(15trunk)/input__cardNumberField'), '神通測試')

WebUI.click(findTestObject('Object Repository/流通/借閱台/清除讀者資料/Page_(15trunk)/input__submitSearch'))

WebUI.click(findTestObject('Object Repository/流通/借閱台/清除讀者資料/Page_(15trunk)/input__RadioGroup'))

WebUI.click(findTestObject('Object Repository/流通/借閱台/清除讀者資料/Page_(15trunk)/input_(15trunk)_Submit'))

WebUI.verifyElementText(findTestObject('Object Repository/流通/借閱台/清除讀者資料/Page_(15trunk)/b_'), '姓名:')

WebUI.verifyElementText(findTestObject('Object Repository/流通/借閱台/清除讀者資料/Page_(15trunk)/a__1_2_3'), '神通測試帳號')

WebUI.verifyElementText(findTestObject('Object Repository/流通/借閱台/清除讀者資料/Page_(15trunk)/b__1'), '代借姓名:')

WebUI.verifyElementText(findTestObject('Object Repository/流通/借閱台/清除讀者資料/Page_(15trunk)/b__1_2'), '借閱證號碼:')

WebUI.verifyElementText(findTestObject('Object Repository/流通/借閱台/清除讀者資料/Page_(15trunk)/b__1_2_3'), '本館借閱量:')

WebUI.verifyElementText(findTestObject('Object Repository/流通/借閱台/清除讀者資料/Page_(15trunk)/b__1_2_3_4'), '已借閱冊數:')

WebUI.verifyElementText(findTestObject('Object Repository/流通/借閱台/清除讀者資料/Page_(15trunk)/b__1_2_3_4_5'), '預約未取:')

WebUI.verifyElementText(findTestObject('Object Repository/流通/借閱台/清除讀者資料/Page_(15trunk)/b__1_2_3_4_5_6'), '逾期停權天數:')

WebUI.verifyElementText(findTestObject('Object Repository/流通/借閱台/清除讀者資料/Page_(15trunk)/b__1_2_3_4_5_6_7'), '賠款總計:')

WebUI.verifyElementText(findTestObject('Object Repository/流通/借閱台/清除讀者資料/Page_(15trunk)/b__1_2_3_4_5_6_7_8'), '逾期罰款:')

WebUI.verifyElementText(findTestObject('Object Repository/流通/借閱台/清除讀者資料/Page_(15trunk)/b__1_2_3_4_5_6_7_8_9'), '設備罰款金額:')

WebUI.click(findTestObject('Object Repository/流通/借閱台/清除讀者資料/Page_(15trunk)/a__1_2_3_4'))

WebUI.verifyElementText(findTestObject('Object Repository/流通/借閱台/清除讀者資料/Page_(15trunk)/p_'), '未選擇讀者資料')

WebUI.takeFullPageScreenshotAsCheckpoint('清除讀者資料')

WebUI.click(findTestObject('Object Repository/流通/借閱台/清除讀者資料/Page_(15trunk)/a__1_2_3_4_5'))

WebUI.closeBrowser()

